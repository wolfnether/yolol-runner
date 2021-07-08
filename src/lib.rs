use std::fs::read_to_string;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

use yolol_devices::devices::chip::CodeRunner;
use yolol_devices::field::Field;
use yolol_devices::value::ValueTrait;
use yolol_devices::value::YololInt;
use yolol_devices::value::YololValue;

#[derive(Debug, Default)]
pub struct YololRunner {
    lines: Vec<String>,
    globals: Vec<Field>,
    locals: Vec<Field>,
    pc: usize,
    path:String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tree {
    Empty,
    Error,
    Comment(String),
    Numerical(i64),
    String(String),
    LocalVariable(String),
    GlobalVariable(String),
    Assign(Box<Tree>, Box<Tree>),
    AssignAdd(Box<Tree>, Box<Tree>),
    AssignSub(Box<Tree>, Box<Tree>),
    AssignMul(Box<Tree>, Box<Tree>),
    AssignDiv(Box<Tree>, Box<Tree>),
    AssignMod(Box<Tree>, Box<Tree>),
    AssignExp(Box<Tree>, Box<Tree>),
    IfThen(Box<Tree>, Vec<Tree>),
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
    Abs(Box<Tree>),
    Sqrt(Box<Tree>),
    Sin(Box<Tree>),
    Cos(Box<Tree>),
    Tan(Box<Tree>),
    Asin(Box<Tree>),
    Acos(Box<Tree>),
    Atan(Box<Tree>),
    Not(Box<Tree>),
    PreDec(Box<Tree>),
    PreInc(Box<Tree>),
    PostDec(Box<Tree>),
    PostInc(Box<Tree>),
    Neg(Box<Tree>),
    Fac(Box<Tree>),
}

peg::parser! {
    grammar yolol_parser() for str{
        #[cache]
        rule nl() = "\n" / "\r\n"
        #[cache]
        rule ss() = (" ")*

        #[cache]
        rule alpha() -> String = n:$(['a'..='z'] / ['A'..='Z'] / ['_']) { n.to_string() }
        #[cache]
        rule digit() -> String = n:$(['0'..='9']) { n.to_string() }
        #[cache]
        rule alphanumeric() -> String = digit() / alpha()

        pub rule line() -> Vec<Tree> = s:(" "* s:stmt() {s})* " "* {s} //ls:( s:stmt() {s})* [_] {let mut s = vec![s]; s.append(&mut ls.clone());s}
        rule stmt() -> Tree = goto() / if_then_end() / (a:assignment() {a}) / comment() / expression()  // "" {Tree::Empty}
        rule goto() -> Tree = "goto" ss() e:expression() {Tree::Goto(e.into())}
        rule if_then_end() -> Tree = "if" ss() p:expression() ss() "then" l:line() ss() "end" {Tree::IfThen(p.into(), l)}
        rule assignment() -> Tree =
            l:variable() ss() "=" ss() r:expression() {Tree::Assign(l.into(), r.into())}
            / l:variable() ss() "+=" ss() r:expression() {Tree::AssignAdd(l.into(), r.into())}
            / l:variable() ss() "-=" ss() r:expression() {Tree::AssignSub(l.into(), r.into())}
            / l:variable() ss() "*=" ss() r:expression() {Tree::AssignMul(l.into(), r.into())}
            / l:variable() ss() "/=" ss() r:expression() {Tree::AssignDiv(l.into(), r.into())}
            / l:variable() ss() "%=" ss() r:expression() {Tree::AssignMod(l.into(), r.into())}
            / l:variable() ss() "^=" ss() r:expression() {Tree::AssignExp(l.into(), r.into())}
        rule expression() -> Tree = precedence!{
            l:@ ss() "and" ss() r:(@) {Tree::And(l.into(), r.into())}
            l:@ ss() "or" ss() r:(@) {Tree::Or(l.into(), r.into())}
            --
            "not" ss() r:(@) {Tree::Not(r.into())}
            --
            l:(@) ss() "+" ss() r:@ {Tree::Add(l.into(), r.into())}
            l:(@) ss() "-" ss() r:@ {Tree::Sub(l.into(), r.into())}
            --
            l:(@) ss() "==" ss() r:@ {Tree::Eq(l.into(), r.into())}
            l:(@) ss() "!=" ss() r:@ {Tree::Ne(l.into(), r.into())}
            l:(@) ss() "<" ss() r:@ {Tree::Lt(l.into(), r.into())}
            l:(@) ss() ">" ss() r:@ {Tree::Gt(l.into(), r.into())}
            l:(@) ss() "<=" ss() r:@ {Tree::Lte(l.into(), r.into())}
            l:(@) ss() ">=" ss() r:@ {Tree::Gte(l.into(), r.into())}
            --
            l:(@) ss() "*" ss() r:@ {Tree::Mul(l.into(), r.into())}
            l:(@) ss() "/" ss() r:@ {Tree::Div(l.into(), r.into())}
            l:(@) ss() "%" ss() r:@ {Tree::Mod(l.into(), r.into())}
            --
            l:@ ss() "^" ss() r:(@) {Tree::Exp(l.into(), r.into())}
            --
            "abs" ss() r:(@) {Tree::Abs(r.into())}
            "sqrt" ss() r:(@) {Tree::Sqrt(r.into())}
            "sin" ss() r:(@) {Tree::Sin(r.into())}
            "asin" ss() r:(@) {Tree::Asin(r.into())}
            "cos" ss() r:(@) {Tree::Cos(r.into())}
            "acos" ss() r:(@) {Tree::Acos(r.into())}
            "tan" ss() r:(@) {Tree::Tan(r.into())}
            "atan" ss() r:(@) {Tree::Atan(r.into())}
            --
            l:litteral() {l}
            "++" ss() r:(@) {Tree::PreInc(r.into())}
            "--" ss() r:(@) {Tree::PreDec(r.into())}
            "-" r:@ {Tree::Neg(r.into())}
            --
            l:(@) ss() "++" {Tree::PostInc(l.into())}
            l:(@) ss() "--" {Tree::PostDec(l.into())}
            --
            l:@ ss() "!" {Tree::Fac(l.into())}
            "(" ss() e:expression() ss() ")" {e}
            v:variable() {v}
        }

        rule comment() -> Tree = "//" c:$(([^'\n']/ [^_])* ) {Tree::Comment(c.to_string())}
        rule variable() -> Tree =
            ":" s:$(b:alphanumeric()*) ) {Tree::GlobalVariable(s.to_string())}
            / !("if" / "end"/ "goto" ) s:$((a:alpha() b:alphanumeric()*)) {Tree::LocalVariable(s.to_string())}
        rule litteral() -> Tree =
            "-" d:$(digit()*) "." r:$(digit()+) {let d : i64 = ("-".to_string()+d).parse().unwrap();let r: i64 = match r.len() {1 => r.parse::<i64>().unwrap() * 100,2 => r.parse::<i64>().unwrap() * 10,_ => r[0..r.len().min(3)].parse().unwrap(),};Tree::Numerical((d * 1000).saturating_sub(r))}
            / "-" d:$(digit()+) {let d : i64 = ("-".to_string()+d).parse().unwrap();Tree::Numerical(d * 1000)}
            / d:$(digit()*) "." r:$(digit()+) {let d : i64 = d.parse().unwrap();let r: i64 = match r.len() {1 => r.parse::<i64>().unwrap() * 100,2 => r.parse::<i64>().unwrap() * 10,_ => r[0..r.len().min(3)].parse().unwrap(),};Tree::Numerical((d * 1000).saturating_add(r))}
            / d:$(digit()+) {let d : i64 = d.parse().unwrap();Tree::Numerical(d * 1000)}
            / "\"" s:$([^ '"']*) "\"" {Tree::String(s.to_string())}
    }
}

impl YololRunner {
    /// Get a reference to the yolol runner's globals.
    pub fn globals(&self) -> &[Field] {
        self.globals.as_slice()
    }

    /// Get a reference to the yolol runner's locals.
    pub fn locals(&self) -> &[Field] {
        self.locals.as_slice()
    }

    fn get_local(&mut self, k: String) -> &mut Field {
        if self.locals.iter().any(|f| f.name() == k) {
            for f in &mut self.locals {
                if f.name() == k {
                    return f;
                }
            }
            unreachable!();
        } else {
            let mut field = Field::default();
            field.set_name(k.clone());
            self.locals.push(field);
            self.get_local(k)
        }
    }

    fn get_global(&mut self, k: String) -> &mut Field {
        if self.globals.iter().any(|f| f.name() == k) {
            for f in &mut self.globals {
                if f.name() == k {
                    return f;
                }
            }
            unreachable!();
        } else {
            let mut field = Field::default();
            field.set_name(k.clone());
            self.globals.push(field);
            self.get_global(k)
        }
    }

    fn process(&mut self, token: &Tree) -> Option<()> {
        match token {
            Tree::Error => None,
            Tree::Comment(_) => Some(()),
            Tree::Assign(r, l) => self.process_assing(r, l),
            Tree::IfThen(p, s) => {
                let v = self.process_expr(p)?;
                if v.into() {
                    for stmt in s {
                        if self.process(stmt).is_none() {
                            return None;
                        }
                    }
                }
                Some(())
            }
            Tree::Goto(t) => {
                if let YololValue::Int(v) = self.process_expr(t)? {
                    let pc: i64 = v.into();
                    self.pc = (pc - 2).clamp(0, 20) as usize;
                }
                None
            }
            Tree::Empty => Some(()),
            t => self.process_expr(t).map(|_| ()),
        }
    }

    fn process_assing(&mut self, r: &Tree, l: &Tree) -> Option<()> {
        let value = self.process_expr(l)?;

        let field = match r {
            Tree::LocalVariable(v) => self.get_local(v.clone()),
            Tree::GlobalVariable(v) => self.get_global(v.clone()),
            t => unreachable!("process_assing : {:?}", t),
        };
        **field = value;
        Some(())
    }
    fn process_expr(&mut self, token: &Tree) -> Option<YololValue> {
        match token {
            Tree::LocalVariable(v) => Some(self.get_local(v.clone()).clone()),
            Tree::GlobalVariable(v) => Some(self.get_global(v.clone()).clone()),
            Tree::Numerical(v) => Some(YololInt::new_raw(*v).into()),
            Tree::String(v) => Some(v.as_str().into()),
            Tree::Or(r, l) => Some(self.process_expr(r)?.or(self.process_expr(l)?)),
            Tree::And(r, l) => Some(self.process_expr(r)?.and(self.process_expr(l)?)),
            Tree::Eq(r, l) => Some((self.process_expr(r)? == self.process_expr(l)?).into()),
            Tree::Ne(r, l) => Some((self.process_expr(r)? != self.process_expr(l)?).into()),
            Tree::Gt(r, l) => Some((self.process_expr(r)? > self.process_expr(l)?).into()),
            Tree::Lt(r, l) => Some((self.process_expr(r)? < self.process_expr(l)?).into()),
            Tree::Gte(r, l) => Some((self.process_expr(r)? >= self.process_expr(l)?).into()),
            Tree::Lte(r, l) => Some((self.process_expr(r)? <= self.process_expr(l)?).into()),
            Tree::Add(r, l) => Some(self.process_expr(r)? + self.process_expr(l)?),
            Tree::Sub(r, l) => self.process_expr(r)? - self.process_expr(l)?,
            Tree::Mul(r, l) => self.process_expr(r)? * self.process_expr(l)?,
            Tree::Div(r, l) => self.process_expr(r)? / self.process_expr(l)?,
            Tree::Mod(r, l) => self.process_expr(r)? % self.process_expr(l)?,
            Tree::Neg(l) => YololValue::default() - self.process_expr(l)?,
            Tree::Fac(r) => self.process_expr(r)?.fac(),
            Tree::Abs(r) => self.process_expr(r)?.abs(),
            Tree::Sqrt(r) => self.process_expr(r)?.sqrt(),
            Tree::Sin(r) => self.process_expr(r)?.sin(),
            Tree::Asin(r) => self.process_expr(r)?.asin(),
            Tree::Cos(r) => self.process_expr(r)?.cos(),
            Tree::Acos(r) => self.process_expr(r)?.acos(),
            Tree::Tan(r) => self.process_expr(r)?.tan(),
            Tree::Atan(r) => self.process_expr(r)?.atan(),
            Tree::Not(r) => Some((self.process_expr(r)? == false.into()).into()),
            Tree::Exp(r, l) => self.process_expr(r)?.pow(self.process_expr(l)?),

            Tree::AssignAdd(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                let v = field.clone().add(v);
                **field = v;
                Some(YololValue::default())
            }

            Tree::AssignSub(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                let v = field.clone().sub(v);
                **field = v?;
                Some(YololValue::default())
            }

            Tree::AssignMul(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                let v = field.clone().mul(v);
                **field = v?;
                Some(YololValue::default())
            }

            Tree::AssignDiv(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                let v = field.clone().div(v);
                **field = v?;
                Some(YololValue::default())
            }

            Tree::AssignMod(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                let v = field.clone().rem(v);
                **field = v?;
                Some(YololValue::default())
            }

            Tree::AssignExp(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                let v = field.clone().pow(v);
                **field = v?;
                Some(YololValue::default())
            }

            Tree::PostInc(r) => {
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                Some(field.post_inc())
            }
            Tree::PostDec(r) => {
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                field.post_dec()
            }
            Tree::PreDec(r) => {
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                field.pre_dec()
            }
            Tree::PreInc(r) => {
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                Some(field.pre_inc())
            }
            t => unreachable!("process_expr : {:?}", t),
        }
    }
}

impl CodeRunner for YololRunner {
    fn parse(&mut self, path: &str) {
        self.path = path.to_string();
        if let Ok(file) = read_to_string(path) {
            self.lines = file
                .replace("\r\n", "\n")
                .split("\n")
                .map(|s| s.to_string())
                .collect(); //yolol_parser::root(&file).unwrap();
        }
    }

    fn step(&mut self) {
        if self.lines.len() <= self.pc {
            self.pc = 0;
        }
        let stmts = yolol_parser::line(&self.lines[self.pc]);
        if let Ok(stmts) = stmts {
            for stmt in stmts {
                if self.process(&stmt).is_none() {
                    break;
                }
            }
        } else if let Err(err) = stmts {
            println!("error {} line {}\n{}", self.path, self.pc+1, err);
        }
        self.pc += 1;
    }
}
