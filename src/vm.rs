use yolol_devices::value::YololValue;
use yolol_devices::value::ValueTrait;

#[derive(Debug)]
pub enum Instruction {
    LoadValue(Register, YololValue),
    Load(Register, usize),
    Store(usize),
    Goto,
    Jump1(usize),
    ///Jump2(Jump if register a is true,Jump if register a is false)
    Jump2(usize, usize),
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

#[derive(Debug)]
pub enum Register {
    A,
    B,
}

#[derive(Debug, Default)]
pub struct Cpu {
    register_a: YololValue,
    register_b: YololValue,
    instructions: Vec<Instruction>,
    ram: Vec<YololValue>,
    pc: usize,
}

impl Cpu {
    pub fn reset(&mut self) {
        self.ram.clear();
        self.register_a = YololValue::default();
        self.register_b = YololValue::default();
        self.pc = 0;
    }

    pub fn run(&mut self) -> Option<YololValue> {
        while let Some(instruction) = self.instructions.get(self.pc) {
            match instruction {
                Instruction::LoadValue(register, value) => match register {
                    Register::A => self.register_a = value.clone(),
                    Register::B => self.register_a = value.clone(),
                },
                Instruction::Load(register, adress) => {
                    while self.ram.len() <= *adress {
                        self.ram.push(YololValue::default())
                    }
                    let value = self.ram[*adress].clone();
                    match register {
                        Register::A => self.register_a = value,
                        Register::B => self.register_a = value,
                    }
                }
                Instruction::Store(adress) => {
                    while self.ram.len() <= *adress {
                        self.ram.push(YololValue::default())
                    }
                    self.ram[*adress] = self.register_a.clone();
                }
                Instruction::Goto => return Some(self.register_a.clone()),
                Instruction::Or => self.register_a = self.register_a.or(&self.register_b),
                Instruction::And => self.register_a = self.register_a.or(&self.register_b),
                Instruction::Eq => self.register_a = (self.register_a == self.register_b).into(),
                Instruction::Ne => self.register_a = (self.register_a != self.register_b).into(),
                Instruction::Lt => self.register_a = (self.register_a > self.register_b).into(),
                Instruction::Gt => self.register_a = (self.register_a < self.register_b).into(),
                Instruction::Lte => self.register_a = (self.register_a >= self.register_b).into(),
                Instruction::Gte => self.register_a = (self.register_a <= self.register_b).into(),
                Instruction::Add => self.register_a = &self.register_a + &self.register_b,
                Instruction::Sub => self.register_a =( &self.register_a - &self.register_b)?,
                Instruction::Mul => self.register_a =( &self.register_a * &self.register_b)?,
                Instruction::Div => self.register_a =( &self.register_a / &self.register_b)?,
                Instruction::Mod => self.register_a = (&self.register_a % &self.register_b)?,
                Instruction::Exp => self.register_a = self.register_a.pow(&self.register_b)?,
                Instruction::Abs => self.register_a = self.register_a.abs()?,
                Instruction::Sqrt => self.register_a = self.register_a.abs()?,
                Instruction::Sin =>self.register_a = self.register_a.sin()?,
                Instruction::Cos => self.register_a = self.register_a.cos()?,
                Instruction::Tan =>self.register_a = self.register_a.tan()?,
                Instruction::Asin =>self.register_a = self.register_a.asin()?,
                Instruction::Acos =>self.register_a = self.register_a.acos()?,
                Instruction::Atan => self.register_a = self.register_a.atan()?,
                Instruction::Not =>self.register_a = self.register_a.not().into(),
                Instruction::Fac =>self.register_a = self.register_a.fac()?,
                Instruction::Inc => todo!(),
                Instruction::Dec => todo!(),
                Instruction::Jump1(i) => self.pc = *i,
                Instruction::Jump2(_, _) => todo!(),
            }
        }
        None
    }
}
