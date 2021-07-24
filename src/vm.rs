use yolol_devices::value::YololValue;

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Instruction {
    Dup,
    Pop,
    PushValue(YololValue),
    Push(usize),
    Store(usize),
    Goto,
    ///jump relative
    Jump(usize),
    ///jump relative if false
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
    pc: isize,
}

impl VM {}
