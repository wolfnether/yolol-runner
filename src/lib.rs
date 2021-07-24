mod ast;
mod parser;
mod vm;

use std::fs::read_to_string;

use ast::Tree;
use lazy_static::__Deref;
use mimalloc::MiMalloc;
use parser::yolol_parser;
use vm::Instruction;
use vm::VM;
use yolol_devices::devices::chip::CodeRunner;
use yolol_devices::field::Field;
use yolol_devices::value::ValueTrait;
use yolol_devices::value::YololValue;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Debug, Default)]
pub struct YololRunner {
    lines: [Vec<Instruction>; 20],
    pc: usize,
    path: String,
    vm: VM,
    variables: Vec<YololValue>,
    stack: Vec<YololValue>,
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
                p
            }
            Tree::IfThenElse(p, t, f) => {
                let mut p = self.process_expr(p);
                let mut t = t
                    .iter()
                    .map(|s| self.process(s))
                    .reduce(|mut a, mut b| {
                        a.append(&mut b);
                        a
                    })
                    .unwrap_or_default();
                let mut f = f
                    .iter()
                    .map(|s| self.process(s))
                    .reduce(|mut a, mut b| {
                        a.append(&mut b);
                        a
                    })
                    .unwrap_or_default();
                p.push(Instruction::JumpFalse(t.len() + 1));
                p.append(&mut t);
                p.push(Instruction::Jump(f.len()));
                p.append(&mut f);
                p
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

        match r {
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
            Tree::String(v) => vec![Instruction::PushValue(v.as_str().into())],
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
                let mut b = self.process_expr(r);
                let mut a = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Add);
                b
            }
            Tree::Sub(r, l) => {
                let mut b = self.process_expr(r);
                let mut a = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Sub);
                b
            }
            Tree::Mul(r, l) => {
                let mut b = self.process_expr(r);
                let mut a = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Mul);
                b
            }
            Tree::Div(r, l) => {
                let mut b = self.process_expr(r);
                let mut a = self.process_expr(l);
                b.append(&mut a);
                b.push(Instruction::Div);
                b
            }
            Tree::Mod(r, l) => {
                let mut b = self.process_expr(r);
                let mut a = self.process_expr(l);
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
                let mut a = self.process_expr(l);
                let mut b = self.process_expr(r);
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
                    Instruction::Inc,
                    Instruction::Dup,
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
                    Instruction::Dec,
                    Instruction::Dup,
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

    pub fn run(&mut self) -> Option<YololValue> {
        self.stack.clear();
        let mut pc = 0;
        let instructions = &self.lines[self.pc];
        while instructions.len() > pc as usize {
            let instruction = &instructions[pc as usize];
            //println!("{:?}", instruction);
            match instruction {
                Instruction::PushValue(value) => self.stack.push(value.clone()),
                Instruction::Push(adress) => {
                    let value = self.variables[*adress].clone();
                    self.stack.push(value);
                }
                Instruction::Store(adress) => {
                    self.variables[*adress] = self.stack.pop()?;
                }
                Instruction::Goto => return self.stack.pop(),
                Instruction::Or => {
                    let b = self.stack.pop()?;
                    let a = self.stack.last_mut()?;
                    *a = a.or(&b)
                }
                Instruction::And => {
                    let b = self.stack.pop()?;
                    let a = self.stack.last_mut()?;
                    *a = a.and(&b)
                }
                Instruction::Eq => {
                    let b = self.stack.pop()?;
                    let a = self.stack.last_mut()?;
                    *a = (*a == b).into()
                }
                Instruction::Ne => {
                    let b = self.stack.pop()?;
                    let a = self.stack.last_mut()?;
                    *a = (*a != b).into()
                }
                Instruction::Lt => {
                    let b = self.stack.pop()?;
                    let a = self.stack.last_mut()?;
                    *a = (*a > b).into()
                }
                Instruction::Gt => {
                    let b = self.stack.pop()?;
                    let a = self.stack.last_mut()?;
                    *a = (*a < b).into()
                }
                Instruction::Lte => {
                    let b = self.stack.pop()?;
                    let a = self.stack.last_mut()?;
                    *a = (*a >= b).into()
                }
                Instruction::Gte => {
                    let b = self.stack.pop()?;
                    let a = self.stack.last_mut()?;
                    *a = (*a <= b).into()
                }
                Instruction::Add => {
                    let b = self.stack.pop()?;
                    let a = self.stack.last_mut()?;
                    *a = &*a + &b
                }
                Instruction::Sub => {
                    let b = self.stack.pop()?;
                    let a = self.stack.last_mut()?;
                    *a = (&*a - &b)?
                }
                Instruction::Mul => {
                    let b = self.stack.pop()?;
                    let a = self.stack.last_mut()?;
                    *a = (&*a * &b)?
                }
                Instruction::Div => {
                    let b = self.stack.pop()?;
                    let a = self.stack.last_mut()?;
                    *a = (&*a / &b)?;
                }
                Instruction::Mod => {
                    let b = self.stack.pop()?;
                    let a = self.stack.last_mut()?;
                    *a = (&*a % &b)?;
                }
                Instruction::Exp => {
                    let b = self.stack.pop()?;
                    let a = self.stack.last_mut()?;
                    *a = a.pow(&b)?;
                }
                Instruction::Abs => {
                    let v = self.stack.last_mut()?;
                    *v = v.abs()?;
                }

                Instruction::Sqrt => {
                    let v = self.stack.last_mut()?;
                    *v = v.sqrt()?;
                }
                Instruction::Sin => {
                    let v = self.stack.last_mut()?;
                    *v = v.sin()?;
                }
                Instruction::Cos => {
                    let v = self.stack.last_mut()?;
                    *v = v.cos()?;
                }
                Instruction::Tan => {
                    let v = self.stack.last_mut()?;
                    *v = v.tan()?;
                }
                Instruction::Asin => {
                    let v = self.stack.last_mut()?;
                    *v = v.asin()?;
                }
                Instruction::Acos => {
                    let v = self.stack.last_mut()?;
                    *v = v.acos()?;
                }
                Instruction::Atan => {
                    let v = self.stack.last_mut()?;
                    *v = v.atan()?;
                }
                Instruction::Not => {
                    let v = self.stack.last_mut()?;
                    *v = v.not();
                }
                Instruction::Fac => {
                    let v = self.stack.last_mut()?;
                    *v = v.fac()?;
                }
                Instruction::Inc => {
                    let mut i = (self.stack.pop()?).clone();
                    i.pre_inc();
                    self.stack.push(i);
                }
                Instruction::Dec => {
                    let mut i = (self.stack.pop()?).clone();
                    i.pre_dec();
                    self.stack.push(i);
                }
                Instruction::Jump(i) => pc = *i as isize,
                Instruction::JumpFalse(i) => {
                    let b: bool = (&self.stack.pop()?).into();
                    if !b {
                        pc += *i as isize;
                    }
                }
                Instruction::Dup => {
                    self.stack.push(self.stack.last()?.clone());
                }
                Instruction::Pop => {
                    self.stack.pop();
                }
            }
            pc += 1;
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Type {
    String,
    Int(Bool),
    Unkown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Bool {
    True,
    False,
    Unknown,
}

fn optimize(mut insts: Vec<Instruction>, ram: &mut [Type]) -> Option<Vec<Instruction>> {
    let mut error = true;
    while error {
        let mut stack: Vec<(usize, Type)> = Vec::new();
        let mut i = 0;
        let mut v = vec![];
        let mut jump = 0;
        error = false;
        for (c, inst) in insts.iter().enumerate() {
            if jump != 0 {
                jump -= 1;
                continue;
            }
            match &inst {
                Instruction::Dup => {
                    let v = stack.pop()?;
                    stack.push((i, v.1));
                    stack.push(v);
                }
                Instruction::Pop => {
                    stack.pop()?;
                }
                Instruction::PushValue(v) => match v {
                    YololValue::String(_) => stack.push((i, Type::String)),
                    YololValue::Int(v) => {
                        if v.into() {
                            stack.push((i, Type::Int(Bool::True)))
                        } else {
                            stack.push((i, Type::Int(Bool::False)))
                        }
                    }
                },
                Instruction::Push(a) => stack.push((i, ram[*a])),
                Instruction::Store(a) => ram[*a] = stack.pop()?.1,
                Instruction::Goto => {
                    if stack.pop()?.1 == Type::String {
                        error = true;
                        break;
                    }
                }
                Instruction::Or => {
                    let (_, a) = stack.pop()?;
                    let (_, b) = stack.pop()?;
                    if a == Type::Int(Bool::Unknown) || b == Type::Int(Bool::Unknown) {
                        stack.push((i, Type::Int(Bool::Unknown)));
                    } else if (a == Type::String || a == Type::Int(Bool::False))
                        && (b == Type::String || b == Type::Int(Bool::False))
                    {
                        stack.push((i, Type::Int(Bool::False)));
                    } else {
                        stack.push((i, Type::Int(Bool::True)));
                    }
                }
                Instruction::And => {
                    let (_, a) = stack.pop()?;
                    let (_, b) = stack.pop()?;
                    if a == Type::Int(Bool::Unknown) || b == Type::Int(Bool::Unknown) {
                        stack.push((i, Type::Int(Bool::Unknown)));
                    } else if (a == Type::String || a == Type::Int(Bool::False))
                        || (b == Type::String || b == Type::Int(Bool::False))
                    {
                        stack.push((i, Type::Int(Bool::False)));
                    } else {
                        stack.push((i, Type::Int(Bool::True)));
                    }
                }
                Instruction::Eq
                | Instruction::Ne
                | Instruction::Lt
                | Instruction::Gt
                | Instruction::Lte
                | Instruction::Gte => {
                    stack.pop()?;
                    stack.pop()?;
                    stack.push((i, Type::Int(Bool::Unknown)));
                }
                Instruction::Add | Instruction::Sub => {
                    let a = stack.pop()?;
                    let b = stack.pop()?;
                    if a.1 == Type::String || b.1 == Type::String {
                        stack.push((i, Type::String));
                    } else {
                        stack.push((i, Type::Int(Bool::Unknown)));
                    }
                }
                Instruction::Mul | Instruction::Div | Instruction::Mod | Instruction::Exp => {
                    let a = stack.pop()?;
                    let b = stack.pop()?;
                    if a.1 == Type::String || b.1 == Type::String {
                        error = true;
                        break;
                    } else {
                        stack.push((i, Type::Int(Bool::Unknown)));
                    }
                }
                Instruction::Abs
                | Instruction::Sqrt
                | Instruction::Sin
                | Instruction::Cos
                | Instruction::Tan
                | Instruction::Asin
                | Instruction::Acos
                | Instruction::Atan
                | Instruction::Fac => {
                    if stack.pop()?.1 == Type::String {
                        error = true;
                        break;
                    } else {
                        stack.push((i, Type::Int(Bool::Unknown)));
                    }
                }
                Instruction::Not => {
                    let (_, t) = stack.pop()?;
                    if t == Type::String {
                        stack.push((i, Type::String));
                    } else if t == Type::Int(Bool::False) {
                        stack.push((i, Type::Int(Bool::True)));
                    } else if t == Type::Int(Bool::True) {
                        stack.push((i, Type::Int(Bool::False)));
                    } else {
                        stack.push((i, Type::Int(Bool::Unknown)));
                    }
                }

                Instruction::Inc | Instruction::Dec => {
                    stack.pop()?;
                    stack.push((i, Type::Int(Bool::Unknown)));
                }
                Instruction::JumpFalse(rel) => {
                    let (_, t) = stack.pop()?;
                    if t == Type::Int(Bool::True) || t == Type::Int(Bool::Unknown) {
                        let mut ret = optimize(insts[c + 1..c + *rel + 1].to_vec(), ram)?;
                        if t == Type::Int(Bool::Unknown) {
                            i += 1;
                            v.push(inst.clone());
                        }

                        i += ret.len();
                        v.append(&mut ret);
                    }
                    jump = *rel;
                    continue;
                }
                _ => (),
            }
            v.push(inst.clone());
            i += 1;
        }
        if !stack.is_empty() {
            error = true;
            for (i, _) in stack.iter().rev() {
                v.remove(*i);
            }
        }
        insts = v;
    }

    Some(insts)
}

impl CodeRunner for YololRunner {
    fn parse(&mut self, path: &str) -> Option<()> {
        self.path = path.to_string();
        if let Ok(file) = read_to_string(path) {
            let mut lines: Vec<Vec<Instruction>> = file
                .replace("\r\n", "\n")
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
                .take(20)
                .collect();
            for (i, line) in lines.iter().enumerate() {
                let mut ram = vec![];
                for _ in 0..*crate::parser::I.lock() {
                    ram.push(Type::Int(Bool::Unknown));
                }
                let line = optimize(line.clone(), &mut ram)?;
                self.lines[i] = line;
            }
            lines = self.lines.to_vec();

            for i in lines.len()..20 {
                self.lines[i] = vec![];
            }

            for _ in 0..*crate::parser::I.lock() {
                self.variables.push(YololValue::default());
            }

            self.stack = Vec::with_capacity(32);
            return Some(());
        }
        None
    }

    fn step(&mut self) {
        if self.pc == 20 {
            self.pc = 0;
        }
        if self.lines[self.pc].is_empty() {
            self.pc += 1;
            return;
        }

        if let Some(YololValue::Int(v)) = self.run() {
            let v: i64 = (&v).into();
            self.pc = (v - 1).clamp(0, 19) as usize;
        } else {
            self.pc += 1;
        }
    }

    fn update_globals(&mut self, globals: Vec<Field>) {
        let mut vec = vec![];
        for global in globals {
            let k = global.name().to_lowercase();
            let adress = crate::parser::GLOBALS.lock()[&k];

            while vec.len() <= adress {
                vec.push(YololValue::default())
            }

            vec[adress] = (*global).clone();
        }
        self.variables = vec;
    }

    fn get_global(&self) -> Vec<Field> {
        let mut v = vec![];
        for (name, adress) in crate::parser::GLOBALS.lock().deref() {
            let mut global = Field::default();
            global.set_name(name.clone());
            if let Some(value) = self.variables.get(*adress) {
                *global = (*value).clone();
            }
            v.push(global);
        }
        v
    }
}
