#[macro_use]
extern crate pest_derive;
use std::fs::read_to_string;

use pest::iterators::Pairs;
use pest::Parser;
use yolol_devices::devices::chip::CodeRunner;
use yolol_devices::Network;
use yolol_devices::Networks;

#[derive(Debug, Default)]
pub struct YololRunner {
    lines: Vec<Vec<Tree>>,
}

#[derive(Debug, PartialEq)]
enum Tree {
    Comment(String),
    Numerical(f64),
    String(String),
    LocalVariable(String),
    GlobalVariable(String),
    Assign(Box<Tree>, Box<Tree>),
    IfThen(Box<Tree>, Box<Tree>),
    Goto(Box<Tree>),
    Or(Box<Tree>, Box<Tree>),
    And(Box<Tree>, Box<Tree>),
    Eq(Box<Tree>, Box<Tree>),
    Ne(Box<Tree>, Box<Tree>),
    Lt(Box<Tree>, Box<Tree>),
    Gt(Box<Tree>, Box<Tree>),
    Lte(Box<Tree>, Box<Tree>),
    Gte(Box<Tree>, Box<Tree>),
    Add(Box<Tree>, Box<Tree>),
    Sub(Box<Tree>, Box<Tree>),
    Mul(Box<Tree>, Box<Tree>),
    Div(Box<Tree>, Box<Tree>),
    Mod(Box<Tree>, Box<Tree>),
    Exp(Box<Tree>, Box<Tree>),
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct YololParser;

impl YololRunner {
    fn parse_line(&self, pairs: Pairs<Rule>) -> Vec<Tree> {
        let mut stmts = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::stmt => stmts.push(self.parse_stmt(pair.into_inner())),
                r => unreachable!("parse_line rule : {:?}", r),
            }
        }
        stmts
    }

    fn parse_stmt(&self, mut pairs: Pairs<Rule>) -> Tree {
        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            Rule::assignment => self.parse_assignment(pair.into_inner()),
            Rule::if_then_end => self.parse_if_then_end(pair.into_inner()),
            Rule::goto => self.parse_goto(pair.into_inner()),
            Rule::comment => Tree::Comment(pair.as_str().into()),
            r => unreachable!("parse_stmt rule : {:?}", r),
        }
    }
    fn parse_goto(&self, mut pairs: Pairs<Rule>) -> Tree {
        Tree::Goto(
            self.parse_logical_operation(pairs.next().unwrap().into_inner())
                .into(),
        )
    }

    fn parse_if_then_end(&self, mut pairs: Pairs<Rule>) -> Tree {
        let pred = self.parse_logical_operation(pairs.next().unwrap().into_inner());
        let stmt = self.parse_stmt(pairs.next().unwrap().into_inner());
        Tree::IfThen(pred.into(), stmt.into())
    }

    fn parse_assignment(&self, mut pairs: Pairs<Rule>) -> Tree {
        let token = self.parse_variable(pairs.next().unwrap().into_inner());
        let pair = pairs.next().unwrap();
        match pair.as_str() {
            "=" => {
                return Tree::Assign(
                    token.into(),
                    self.parse_logical_operation(pairs.next().unwrap().into_inner())
                        .into(),
                )
            }
            r => unreachable!("parse_assignment rule : {:?}", r),
        }
    }

    fn parse_variable(&self, mut pairs: Pairs<Rule>) -> Tree {
        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            Rule::local_variable => Tree::LocalVariable(pair.as_str().into()),
            Rule::global_variable => Tree::GlobalVariable(pair.as_str().into()),
            r => unreachable!("parse_variable rule : {:?}", r),
        }
    }

    fn parse_logical_operation(&self, mut pairs: Pairs<Rule>) -> Tree {
        let pair = pairs.next().unwrap();
        let mut token = match pair.as_rule() {
            Rule::additive_expression => self.parse_additive_expression(pair.into_inner()),
            r => unreachable!("parse_logical_operation rule : {:?}", r),
        };
        while let Some(op) = pairs.next() {
            let rhs = self.parse_additive_expression(pairs.next().unwrap().into_inner());
            token = match op.as_str() {
                "or" => Tree::Or(token.into(), rhs.into()),
                "and" => Tree::And(token.into(), rhs.into()),
                "==" => Tree::Eq(token.into(), rhs.into()),
                "!=" => Tree::Ne(token.into(), rhs.into()),
                ">=" => Tree::Gte(token.into(), rhs.into()),
                "<=" => Tree::Lte(token.into(), rhs.into()),
                ">" => Tree::Gt(token.into(), rhs.into()),
                "<" => Tree::Lt(token.into(), rhs.into()),
                r => unreachable!("parse_variable rule : {:?}", r),
            };
        }
        token
    }
    fn parse_additive_expression(&self, mut pairs: Pairs<Rule>) -> Tree {
        let pair = pairs.next().unwrap();
        let mut token = match pair.as_rule() {
            Rule::multiplicative_expression => {
                self.parse_multiplicative_expression(pair.into_inner())
            }
            r => unreachable!("parse_additive_expression rule : {:?}", r),
        };
        while let Some(op) = pairs.next() {
            let rhs = self.parse_multiplicative_expression(pairs.next().unwrap().into_inner());
            token = match op.as_str() {
                "+" => Tree::Add(token.into(), rhs.into()),
                "-" => Tree::Sub(token.into(), rhs.into()),
                r => unreachable!("parse_variable rule : {:?}", r),
            };
        }
        token
    }
    fn parse_multiplicative_expression(&self, mut pairs: Pairs<Rule>) -> Tree {
        let pair = pairs.next().unwrap();

        let mut token = match pair.as_rule() {
            Rule::exponentiative_expression => {
                self.parse_exponentiative_expression(pair.into_inner())
            }
            r => unreachable!("parse_multiplicative_expression rule : {:?}", r),
        };
        while let Some(op) = pairs.next() {
            let rhs = self.parse_exponentiative_expression(pairs.next().unwrap().into_inner());
            token = match op.as_str() {
                "*" => Tree::Mul(token.into(), rhs.into()),
                "/" => Tree::Div(token.into(), rhs.into()),
                "%" => Tree::Mod(token.into(), rhs.into()),
                r => unreachable!("parse_variable rule : {:?}", r),
            };
        }
        token
    }
    fn parse_exponentiative_expression(&self, mut pairs: Pairs<Rule>) -> Tree {
        let pair = pairs.next().unwrap();

        let mut token = match pair.as_rule() {
            Rule::primary => self.parse_primary(pair.into_inner()),
            r => unreachable!("parse_exponentiative_expression rule : {:?}", r),
        };
        while let Some(op) = pairs.next() {
            let rhs = self.parse_primary(pairs.next().unwrap().into_inner());
            token = match op.as_str() {
                "^" => Tree::Exp(token.into(), rhs.into()),
                r => unreachable!("parse_variable rule : {:?}", r),
            };
        }
        token
    }
    fn parse_primary(&self, mut pairs: Pairs<Rule>) -> Tree {
        let pair = pairs.next().unwrap();

        match pair.as_rule() {
            Rule::logical_operation => self.parse_logical_operation(pair.into_inner()),
            Rule::variable => self.parse_variable(pair.into_inner()),
            Rule::numerical => Tree::Numerical(pair.as_str().parse().unwrap()),
            Rule::string => Tree::String(pair.as_str().into()),
            r => unreachable!("parse_primary rule : {:?}", r),
        }
    }
}

impl CodeRunner for YololRunner {
    fn parse(&mut self, path: &str) {
        if let Ok(file) = read_to_string("Quad.yolol") {
            let pairs =
                YololParser::parse(Rule::grammar_rules, &file).unwrap_or_else(|e| panic!("{}", e));
            for pair in pairs {
                match pair.as_rule() {
                    Rule::line => self.lines.push(self.parse_line(pair.into_inner())),
                    Rule::EOI => (),
                    _ => unreachable!("{:?}", pair.as_rule()),
                }
            }
        }
    }

    fn step(&self, networks: &mut Networks<Self>, network: &Network<Self>) {
        todo!()
    }
}
