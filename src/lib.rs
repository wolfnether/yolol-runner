mod ast;
mod parser;
use std::fs::read_to_string;

use ast::Tree;
use parser::yolol_parser;
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
    path: String,
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
        if self
            .locals
            .iter()
            .any(|f| f.name().to_lowercase() == k.to_lowercase())
        {
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
        if self
            .globals
            .iter()
            .any(|f| f.name().to_lowercase() == k.to_lowercase())
        {
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
                let v = &self.process_expr(p)?;
                if v.into() {
                    for stmt in s {
                        self.process(stmt)?;
                    }
                }
                Some(())
            }
            Tree::Goto(t) => {
                if let YololValue::Int(v) = &self.process_expr(t)? {
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
            Tree::LocalVariable(v) => Some((**self.get_local(v.clone())).clone()),
            Tree::GlobalVariable(v) => Some((**self.get_global(v.clone())).clone()),
            Tree::Numerical(v) => Some(YololInt::new_raw(*v).into()),
            Tree::String(v) => Some(v.as_str().into()),
            Tree::Or(r, l) => Some(self.process_expr(r)?.or(&self.process_expr(l)?)),
            Tree::And(r, l) => Some(self.process_expr(r)?.and(&self.process_expr(l)?)),
            Tree::Eq(r, l) => Some((self.process_expr(r)? == self.process_expr(l)?).into()),
            Tree::Ne(r, l) => Some((self.process_expr(r)? != self.process_expr(l)?).into()),
            Tree::Gt(r, l) => Some((self.process_expr(r)? > self.process_expr(l)?).into()),
            Tree::Lt(r, l) => Some((self.process_expr(r)? < self.process_expr(l)?).into()),
            Tree::Gte(r, l) => Some((self.process_expr(r)? >= self.process_expr(l)?).into()),
            Tree::Lte(r, l) => Some((self.process_expr(r)? <= self.process_expr(l)?).into()),
            Tree::Add(r, l) => Some(&self.process_expr(r)? + &self.process_expr(l)?),
            Tree::Sub(r, l) => &self.process_expr(r)? - &self.process_expr(l)?,
            Tree::Mul(r, l) => &self.process_expr(r)? * &self.process_expr(l)?,
            Tree::Div(r, l) => &self.process_expr(r)? / &self.process_expr(l)?,
            Tree::Mod(r, l) => &self.process_expr(r)? % &self.process_expr(l)?,
            Tree::Neg(l) => &YololValue::default() - &self.process_expr(l)?,
            Tree::Fac(r) => Some(self.process_expr(r)?.fac()?),
            Tree::Abs(r) => Some(self.process_expr(r)?.abs()?),
            Tree::Sqrt(r) => Some(self.process_expr(r)?.sqrt()?),
            Tree::Sin(r) => Some(self.process_expr(r)?.sin()?),
            Tree::Asin(r) => Some(self.process_expr(r)?.asin()?),
            Tree::Cos(r) => Some(self.process_expr(r)?.cos()?),
            Tree::Acos(r) => Some(self.process_expr(r)?.acos()?),
            Tree::Tan(r) => Some(self.process_expr(r)?.tan()?),
            Tree::Atan(r) => Some(self.process_expr(r)?.atan()?),
            Tree::Not(r) => Some((self.process_expr(r)? == false.into()).into()),
            Tree::Exp(r, l) => Some(self.process_expr(r)?.pow(&self.process_expr(l)?)?),

            Tree::AssignAdd(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                **field = &**field + &v;
                Some(YololValue::default())
            }

            Tree::AssignSub(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                **field = (&**field - &v)?;
                Some(YololValue::default())
            }

            Tree::AssignMul(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                **field = (&**field * &v)?;
                Some(YololValue::default())
            }

            Tree::AssignDiv(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                **field = (&**field / &v)?;
                Some(YololValue::default())
            }

            Tree::AssignMod(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                **field = (&**field % &v)?;
                Some(YololValue::default())
            }

            Tree::AssignExp(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                let v = field.pow(&v);
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
                Some(field.post_dec()?)
            }
            Tree::PreDec(r) => {
                let field = match &**r {
                    Tree::LocalVariable(v) => self.get_local(v.to_string()),
                    Tree::GlobalVariable(v) => self.get_global(v.to_string()),
                    _ => unreachable!(),
                };
                Some(field.pre_dec()?)
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
    fn parse(&mut self, path: &str) -> Option<()> {
        self.path = path.to_string();
        if let Ok(file) = read_to_string(path) {
            self.lines = file
                .replace("\r\n", "\n")
                .split('\n')
                .map(|s| s.to_string())
                .collect(); //yolol_parser::root(&file).unwrap();
            return Some(());
        }
        None
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
            println!("error {} line {}\n{}", self.path, self.pc + 1, err);
        }
        self.pc += 1;
    }

    fn update_globals(&mut self, globals: Vec<Field>) {
        self.globals = globals;
    }

    fn get_global(&self) -> Vec<Field> {
        self.globals.clone()
    }
}
