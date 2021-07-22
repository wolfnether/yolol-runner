mod ast;
mod parser;
mod vm;

use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::rc::Rc;

use ast::Tree;
use parser::yolol_parser;
use vm::Instruction;
use vm::VM;
use yolol_devices::devices::chip::CodeRunner;
use yolol_devices::field::Field;
use yolol_devices::value::ValueTrait;
use yolol_devices::value::YololValue;

#[derive(Debug, Default)]
pub struct YololRunner {
    lines: Rc<Vec<Vec<Instruction>>>,
    variables: Vec<YololValue>,
    pc: usize,
    path: String,
    vm: VM,
}

impl YololRunner {
    fn process(&mut self, token: &Tree) -> Vec<Instruction> {
        match token {
            Tree::Error => vec![],
            Tree::Comment(_) => vec![],
            Tree::Assign(r, l) => self.process_assing(r, l),
            Tree::IfThen(p, s) => {
                let mut p = self.process_expr(p);
                let mut s = s
                    .iter()
                    .map(|s| self.process(s))
                    .reduce(|mut a, mut b| {
                        a.append(&mut b);
                        a
                    })
                    .unwrap_or_default();
                p.push(Instruction::JumpFalse(s.len()));
                p.append(&mut s);
                s
            }
            Tree::Goto(t) => {
                let mut v = self.process_expr(t);
                v.push(Instruction::Goto);
                v
            }
            Tree::Empty => vec![],
            t => self.process_expr(t),
        }
    }

    fn process_assing(&mut self, r: &Tree, l: &Tree) -> Vec<Instruction> {
        let mut value = self.process_expr(l);

        let field = match r {
            Tree::LocalVariable(v) | Tree::GlobalVariable(v) => value.push(Instruction::Store(*v)),
            t => unreachable!("process_assing : {:?}", t),
        };

        value
    }

    fn process_expr(&mut self, token: &Tree) -> Vec<Instruction> {
        match token {
            Tree::LocalVariable(v) => vec![Instruction::Push(*v)],
            Tree::GlobalVariable(v) => vec![Instruction::Push(*v)],
            Tree::Numerical(v) => vec![Instruction::PushValue((*v as f64 / 1000.).into())],
            Tree::String(v) => vec![Instruction::PushValue((v.as_str().into()))],
            Tree::Or(r, l) => {
                let mut a = self.process_expr(r);
                let mut b = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Or);
                b
            }
            Tree::And(r, l) => {
                let mut a = self.process_expr(r);
                let mut b = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::And);
                b
            }
            Tree::Eq(r, l) => {
                let mut a = self.process_expr(r);
                let mut b = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Eq);
                b
            }
            Tree::Ne(r, l) => {
                let mut a = self.process_expr(r);
                let mut b = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Ne);
                b
            }
            Tree::Gt(r, l) => {
                let mut a = self.process_expr(r);
                let mut b = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Gt);
                b
            }
            Tree::Lt(r, l) => {
                let mut a = self.process_expr(r);
                let mut b = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Lt);
                b
            }
            Tree::Gte(r, l) => {
                let mut a = self.process_expr(r);
                let mut b = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Gte);
                b
            }
            Tree::Lte(r, l) => {
                let mut a = self.process_expr(r);
                let mut b = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Lte);
                b
            }
            Tree::Add(r, l) => {
                let mut a = self.process_expr(r);
                let mut b = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Add);
                b
            }
            Tree::Sub(r, l) => {
                let mut a = self.process_expr(r);
                let mut b = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Sub);
                b
            }
            Tree::Mul(r, l) => {
                let mut a = self.process_expr(r);
                let mut b = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Mul);
                b
            }
            Tree::Div(r, l) => {
                let mut a = self.process_expr(r);
                let mut b = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Div);
                b
            }
            Tree::Mod(r, l) => {
                let mut a = self.process_expr(r);
                let mut b = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Mod);
                b
            }
            Tree::Neg(l) => {
                let mut a = self.process_expr(l);
                a.push(Instruction::PushValue(YololValue::from(-1)));
                a.push(Instruction::Mul);
                a
            }
            Tree::Fac(r) => {
                let mut a = self.process_expr(r);
                a.push(Instruction::Fac);
                a
            }
            Tree::Abs(r) => {
                let mut a = self.process_expr(r);
                a.push(Instruction::Abs);
                a
            }
            Tree::Sqrt(r) => {
                let mut a = self.process_expr(r);
                a.push(Instruction::Sqrt);
                a
            }
            Tree::Sin(r) => {
                let mut a = self.process_expr(r);
                a.push(Instruction::Sin);
                a
            }
            Tree::Asin(r) => {
                let mut a = self.process_expr(r);
                a.push(Instruction::Asin);
                a
            }
            Tree::Cos(r) => {
                let mut a = self.process_expr(r);
                a.push(Instruction::Cos);
                a
            }
            Tree::Acos(r) => {
                let mut a = self.process_expr(r);
                a.push(Instruction::Acos);
                a
            }
            Tree::Tan(r) => {
                let mut a = self.process_expr(r);
                a.push(Instruction::Tan);
                a
            }
            Tree::Atan(r) => {
                let mut a = self.process_expr(r);
                a.push(Instruction::Atan);
                a
            }
            Tree::Not(r) => {
                let mut a = self.process_expr(r);
                a.push(Instruction::Not);
                a
            }
            Tree::Exp(r, l) => {
                let mut a = self.process_expr(r);
                let mut b = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Exp);
                b
            }

            Tree::AssignAdd(r, l) => {
                let mut v = self.process_expr(l);
                let addr = match **r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => v,
                    _ => unreachable!(),
                };
                v.push(Instruction::Push(addr));
                v.push(Instruction::Add);
                v.push(Instruction::Store(addr));
                v
            }

            Tree::AssignSub(r, l) => {
                let mut v = self.process_expr(l);
                let addr = match **r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => v,
                    _ => unreachable!(),
                };
                v.push(Instruction::Push(addr));
                v.push(Instruction::Sub);
                v.push(Instruction::Store(addr));
                v
            }

            Tree::AssignMul(r, l) => {
                let mut v = self.process_expr(l);
                let addr = match **r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => v,
                    _ => unreachable!(),
                };
                v.push(Instruction::Push(addr));
                v.push(Instruction::Mul);
                v.push(Instruction::Store(addr));
                v
            }

            Tree::AssignDiv(r, l) => {
                let mut v = self.process_expr(l);
                let addr = match **r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => v,
                    _ => unreachable!(),
                };
                v.push(Instruction::Push(addr));
                v.push(Instruction::Div);
                v.push(Instruction::Store(addr));
                v
            }

            Tree::AssignMod(r, l) => {
                let mut v = self.process_expr(l);
                let addr = match **r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => v,
                    _ => unreachable!(),
                };
                v.push(Instruction::Push(addr));
                v.push(Instruction::Mod);
                v.push(Instruction::Store(addr));
                v
            }

            Tree::AssignExp(r, l) => {
                let mut v = self.process_expr(l);
                let addr = match **r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => v,
                    _ => unreachable!(),
                };
                v.push(Instruction::Push(addr));
                v.push(Instruction::Exp);
                v.push(Instruction::Store(addr));
                v
            }

            Tree::PostInc(r) => {
                let addr = match &**r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => *v,
                    _ => unreachable!(),
                };
                vec![
                    Instruction::Push(addr),
                    Instruction::Dup,
                    Instruction::Inc,
                    Instruction::Store(addr),
                ]
            }
            Tree::PostDec(r) => {
                let addr = match &**r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => *v,
                    _ => unreachable!(),
                };
                vec![
                    Instruction::Push(addr),
                    Instruction::Dup,
                    Instruction::Dec,
                    Instruction::Store(addr),
                ]
            }
            Tree::PreDec(r) => {
                let addr = match &**r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => *v,
                    _ => unreachable!(),
                };
                vec![
                    Instruction::Push(addr),
                    Instruction::Inc,
                    Instruction::Dup,
                    Instruction::Store(addr),
                ]
            }
            Tree::PreInc(r) => {
                let addr = match &**r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => *v,
                    _ => unreachable!(),
                };
                vec![
                    Instruction::Push(addr),
                    Instruction::Dec,
                    Instruction::Dup,
                    Instruction::Store(addr),
                ]
            }
            t => unreachable!("process_expr : {:?}", t),
        }
    }
}

impl CodeRunner for YololRunner {
    fn parse(&mut self, path: &str) -> Option<()> {
        self.path = path.to_string();
        if let Ok(file) = read_to_string(path) {
            self.lines = Rc::new(
                file.replace("\r\n", "\n")
                    .split('\n')
                    .map(|s| match yolol_parser::line(s) {
                        Ok(line) => line
                            .iter()
                            .map(|s| self.process(s))
                            .reduce(|mut a, mut b| {
                                a.append(&mut b);
                                a
                            })
                            .unwrap_or_default(),
                        Err(err) => {
                            println!("error {} line {}\n{}", self.path, self.pc + 1, err);
                            vec![]
                        }
                    })
                    .collect(),
            );
            for _ in 0..*crate::parser::I.lock() {
                self.variables.push(YololValue::default());
            }
            return Some(());
        }
        None
    }

    fn step(&mut self) {
        if self.lines.len() <= self.pc {
            self.pc = 0;
        }

        let instructions = &self.lines.clone()[self.pc];
        if let Some(goto) = self.vm.run(instructions) {
            if let YololValue::Int(v) = goto {
                let v: i64 = (&v).into();
                self.pc = v.clamp(0, 20) as usize;
            }
        }
        /* else if let Err(err) = stmts {
            println!("error {} line {}\n{}", self.path, self.pc + 1, err);
        }*/
        self.pc += 1;
    }

    fn update_globals(&mut self, globals: Vec<Field>) {
        /*self.globals.clear();
        for global in globals {
            self.globals
                .insert(global.name().to_lowercase(), (*global).clone());
        }*/
        //self.globals = globals;
    }

    fn get_global(&self) -> Vec<Field> {
        let mut v = vec![];
        /*for (name, value) in &self.globals {
            let mut global = Field::default();
            global.set_name(name.clone());
            *global = value.clone();
            v.push(global);
        }*/
        v
    }
}
