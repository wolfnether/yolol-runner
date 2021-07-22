use std::collections::VecDeque;

use yolol_devices::value::ValueTrait;
use yolol_devices::value::YololValue;

#[derive(Debug)]
pub enum Instruction {
    Dup,
    Pop,
    PushValue(YololValue),
    Push(usize),
    Store(usize),
    Goto,
    Jump1(usize),
    JumpFalse(usize),
    Or,
    And,
    Eq,
    Ne,
    Lt,
    Gt,
    Lte,
    Gte,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
    Abs,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Not,
    Fac,
    Inc,
    Dec,
}

#[derive(Debug, Default)]
pub struct VM {
    stack: VecDeque<YololValue>,
    instructions: Vec<Instruction>,
    ram: Vec<YololValue>,
    pc: isize,
}

impl VM {
    pub fn reset(&mut self) {
        self.ram.clear();
        self.stack.clear();
    }

    pub fn run(&mut self, instructions: &Vec<Instruction>) -> Option<YololValue> {
        self.stack.clear();
        self.pc = 0;
        while let Some(instruction) = instructions.get(self.pc as usize) {
            match instruction {
                Instruction::PushValue(value) => self.stack.push_back(value.clone()),
                Instruction::Push(adress) => {
                    while self.ram.len() <= *adress {
                        self.ram.push(YololValue::default())
                    }
                    let value = self.ram[*adress].clone();
                    self.stack.push_back(value.clone());
                }
                Instruction::Store(adress) => {
                    while self.ram.len() <= *adress {
                        self.ram.push(YololValue::default())
                    }
                    self.ram[*adress] = self.stack.pop_back()?;
                }
                Instruction::Goto => return Some(self.stack.pop_back()?),
                Instruction::Or => {
                    let b = self.stack.pop_back()?;
                    let a = self.stack.pop_back()?;
                    self.stack.push_back(a.or(&b))
                }
                Instruction::And => {
                    let b = self.stack.pop_back()?;
                    let a = self.stack.pop_back()?;
                    self.stack.push_back(a.and(&b))
                }
                Instruction::Eq => {
                    let b = self.stack.pop_back()?;
                    let a = self.stack.pop_back()?;
                    self.stack.push_back((a == b).into())
                }
                Instruction::Ne => {
                    let b = self.stack.pop_back()?;
                    let a = self.stack.pop_back()?;
                    self.stack.push_back((a != b).into())
                }
                Instruction::Lt => {
                    let b = self.stack.pop_back()?;
                    let a = self.stack.pop_back()?;
                    self.stack.push_back((a > b).into())
                }
                Instruction::Gt => {
                    let b = self.stack.pop_back()?;
                    let a = self.stack.pop_back()?;
                    self.stack.push_back((a < b).into())
                }
                Instruction::Lte => {
                    let b = self.stack.pop_back()?;
                    let a = self.stack.pop_back()?;
                    self.stack.push_back((a >= b).into())
                }
                Instruction::Gte => {
                    let b = self.stack.pop_back()?;
                    let a = self.stack.pop_back()?;
                    self.stack.push_back((a <= b).into())
                }
                Instruction::Add => {
                    let b = self.stack.pop_back()?;
                    let a = self.stack.pop_back()?;
                    self.stack.push_back(&a + &b)
                }
                Instruction::Sub => {
                    let b = self.stack.pop_back()?;
                    let a = self.stack.pop_back()?;
                    self.stack.push_back((&a - &b)?)
                }
                Instruction::Mul => {
                    let b = self.stack.pop_back()?;
                    let a = self.stack.pop_back()?;
                    self.stack.push_back((&a * &b)?)
                }
                Instruction::Div => {
                    let b = self.stack.pop_back()?;
                    let a = self.stack.pop_back()?;
                    self.stack.push_back((&a / &b)?)
                }
                Instruction::Mod => {
                    let b = self.stack.pop_back()?;
                    let a = self.stack.pop_back()?;
                    self.stack.push_back((&a % &b)?)
                }
                Instruction::Exp => {
                    let b = self.stack.pop_back()?;
                    let a = self.stack.pop_back()?;
                    self.stack.push_back(a.pow(&b)?);
                }
                Instruction::Abs => {
                    let v = self.stack.back()?;
                    *self.stack.back_mut()? = v.abs()?;
                }

                Instruction::Sqrt => {
                    let v = self.stack.back()?;
                    *self.stack.back_mut()? = v.sqrt()?;
                }
                Instruction::Sin => {
                    let v = self.stack.back()?;
                    *self.stack.back_mut()? = v.sin()?;
                }
                Instruction::Cos => {
                    let v = self.stack.back()?;
                    *self.stack.back_mut()? = v.cos()?;
                }
                Instruction::Tan => {
                    let v = self.stack.back()?;
                    *self.stack.back_mut()? = v.tan()?;
                }
                Instruction::Asin => {
                    let v = self.stack.back()?;
                    *self.stack.back_mut()? = v.asin()?;
                }
                Instruction::Acos => {
                    let v = self.stack.back()?;
                    *self.stack.back_mut()? = v.acos()?;
                }
                Instruction::Atan => {
                    let v = self.stack.back()?;
                    *self.stack.back_mut()? = v.atan()?;
                }
                Instruction::Not => {
                    let v = self.stack.back()?;
                    *self.stack.back_mut()? = v.not().into();
                }
                Instruction::Fac => {
                    let v = self.stack.back()?;
                    *self.stack.back_mut()? = v.fac()?;
                }
                Instruction::Inc => {
                    self.stack.back_mut()?.pre_inc();
                }
                Instruction::Dec => {
                    self.stack.back_mut()?.pre_dec()?;
                }
                Instruction::Jump1(i) => self.pc = *i as isize - 1,
                Instruction::JumpFalse(i) => {
                    if self.stack.pop_back()? == true.into() {
                        self.pc = *i as isize - 1;
                    }
                }
                Instruction::Dup => {
                    self.stack.push_back(self.stack.back()?.clone());
                }
                Instruction::Pop => {
                    self.stack.pop_back();
                }
            }
            self.pc += 1;
        }
        None
    }
}
