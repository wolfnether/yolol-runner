mod ast;
mod parser;

use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::rc::Rc;

use ast::Tree;
use parser::yolol_parser;
use yolol_devices::devices::chip::CodeRunner;
use yolol_devices::field::Field;
use yolol_devices::value::ValueTrait;
use yolol_devices::value::YololValue;

#[derive(Debug, Default)]
pub struct YololRunner {
    lines: Rc<Vec<Vec<Tree>>>,
    variables: Vec<YololValue>,
    pc: usize,
    path: String,
}

impl YololRunner {
    /*/// Get a reference to the yolol runner's globals.
    pub fn globals(&self) -> &[Field] {
        self.globals.as_slice()
    }

    /// Get a reference to the yolol runner's locals.
    pub fn locals(&self) -> &[Field] {
        self.locals.as_slice()
    }*/

    /*///get local variables from VM
    pub fn get_local(&mut self, k: &str) -> &mut YololValue {
        let k = k.to_lowercase();
        if self.locals.contains_key(&k) {
            self.locals.get_mut(&k).unwrap()
        } else {
            let k = k.to_lowercase();
            self.locals.insert(k.clone(), YololValue::default());
            self.locals.get_mut(&k).unwrap()
        }
    }

    ///get global variables from VM
    pub fn get_global(&mut self, k: &str) -> &mut YololValue {
        let k = k.to_lowercase();
        if self.globals.contains_key(&k) {
            self.globals.get_mut(&k).unwrap()
        } else {
            self.globals.insert(k.clone(), YololValue::default());
            self.globals.get_mut(&k).unwrap()
        }
    }*/

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
                Some(())
            }
            Tree::Empty => Some(()),
            t => self.process_expr(t).map(|_| ()),
        }
    }

    fn process_assing(&mut self, r: &Tree, l: &Tree) -> Option<()> {
        let value = self.process_expr(l)?;

        let field = match r {
            Tree::LocalVariable(v) | Tree::GlobalVariable(v)=> &mut self.variables[*v],
            t => unreachable!("process_assing : {:?}", t),
        };
        *field = value;
        Some(())
    }

    fn process_expr(&mut self, token: &Tree) -> Option<YololValue> {
        match token {
            Tree::LocalVariable(v) => Some(self.variables[*v].clone()),
            Tree::GlobalVariable(v) => Some(self.variables[*v].clone()),
            Tree::Numerical(v) => Some((*v as f64 / 1000.).into()),
            Tree::String(v) => Some(v.as_str().into()),
            Tree::Or(r, l) => Some(self.process_expr(r)?.or(&self.process_expr(l)?)),
            Tree::And(r, l) => Some(self.process_expr(r)?.and(&self.process_expr(l)?)),
            Tree::Eq(r, l) => Some(YololValue::from(
                self.process_expr(r)? == self.process_expr(l)?,
            )),
            Tree::Ne(r, l) => Some(YololValue::from(
                self.process_expr(r)? != self.process_expr(l)?,
            )),
            Tree::Gt(r, l) => Some(YololValue::from(
                self.process_expr(r)? > self.process_expr(l)?,
            )),
            Tree::Lt(r, l) => Some(YololValue::from(
                self.process_expr(r)? < self.process_expr(l)?,
            )),
            Tree::Gte(r, l) => Some(YololValue::from(
                self.process_expr(r)? >= self.process_expr(l)?,
            )),
            Tree::Lte(r, l) => Some(YololValue::from(
                self.process_expr(r)? <= self.process_expr(l)?,
            )),
            Tree::Add(r, l) => Some(&self.process_expr(r)? + &self.process_expr(l)?),
            Tree::Sub(r, l) => &self.process_expr(r)? - &self.process_expr(l)?,
            Tree::Mul(r, l) => &self.process_expr(r)? * &self.process_expr(l)?,
            Tree::Div(r, l) => &self.process_expr(r)? / &self.process_expr(l)?,
            Tree::Mod(r, l) => &self.process_expr(r)? % &self.process_expr(l)?,
            Tree::Neg(l) => Some((&YololValue::from(-1) * &self.process_expr(l)?)?),
            Tree::Fac(r) => Some(self.process_expr(r)?.fac()?),
            Tree::Abs(r) => Some(self.process_expr(r)?.abs()?),
            Tree::Sqrt(r) => Some(self.process_expr(r)?.sqrt()?),
            Tree::Sin(r) => Some(self.process_expr(r)?.sin()?),
            Tree::Asin(r) => Some(self.process_expr(r)?.asin()?),
            Tree::Cos(r) => Some(self.process_expr(r)?.cos()?),
            Tree::Acos(r) => Some(self.process_expr(r)?.acos()?),
            Tree::Tan(r) => Some(self.process_expr(r)?.tan()?),
            Tree::Atan(r) => Some(self.process_expr(r)?.atan()?),
            Tree::Not(r) => Some(self.process_expr(r)?.not()),
            Tree::Exp(r, l) => Some(self.process_expr(r)?.pow(&self.process_expr(l)?)?),

            Tree::AssignAdd(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => &mut self.variables[*v],
                    _ => unreachable!(),
                };
                *field = &*field + &v;
                Some(YololValue::default())
            }

            Tree::AssignSub(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => &mut self.variables[*v],
                    _ => unreachable!(),
                };
                *field = (&*field - &v)?;
                Some(YololValue::default())
            }

            Tree::AssignMul(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) =>&mut self.variables[*v],
                    _ => unreachable!(),
                };
                *field = (&*field * &v)?;
                Some(YololValue::default())
            }

            Tree::AssignDiv(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => &mut self.variables[*v],
                    _ => unreachable!(),
                };
                *field = (&*field / &v)?;
                Some(YololValue::default())
            }

            Tree::AssignMod(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) =>&mut self.variables[*v],
                    _ => unreachable!(),
                };
                *field = (&*field % &v)?;
                Some(YololValue::default())
            }

            Tree::AssignExp(r, l) => {
                let v = self.process_expr(l)?;
                let field = match &**r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) =>&mut self.variables[*v],
                    _ => unreachable!(),
                };
                let v = field.pow(&v);
                *field = v?;
                Some(YololValue::default())
            }

            Tree::PostInc(r) => {
                let field = match &**r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) =>&mut self.variables[*v],
                    _ => unreachable!(),
                };
                Some(field.post_inc())
            }
            Tree::PostDec(r) => {
                let field = match &**r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => &mut self.variables[*v],
                    _ => unreachable!(),
                };
                Some(field.post_dec()?)
            }
            Tree::PreDec(r) => {
                let field = match &**r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => &mut self.variables[*v],
                    _ => unreachable!(),
                };
                Some(field.pre_dec()?)
            }
            Tree::PreInc(r) => {
                let field = match &**r {
                    Tree::LocalVariable(v) | Tree::GlobalVariable(v) => &mut self.variables[*v],
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
            self.lines = Rc::new(
                file.replace("\r\n", "\n")
                    .split('\n')
                    .map(|s| {
                        let line = yolol_parser::line(s);
                        if let Ok(line) = line {
                            line
                        } else if let Err(err) = line {
                            println!("error {} line {}\n{}", self.path, self.pc + 1, err);
                            vec![]
                        } else {
                            vec![]
                        }
                    })
                    .collect(),
            );
            for _ in 0..*crate::parser::I.lock(){
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

        let stmts = &self.lines.clone()[self.pc];
        for stmt in stmts {
            if self.process(&stmt).is_none() {
                break;
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
