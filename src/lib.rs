use yolol_devices::{devices::chip::CodeRunner, value::YololValue};

pub struct YololRunner{
    line:Vec<Vec<TokenTree>>
}


pub enum TokenTree{
    Literal(YololValue),
    LocalVariable(String),
    GlobalVariable(String),
    Add(Box<TokenTree>, Box<TokenTree>),
    Sub(Box<TokenTree>, Box<TokenTree>),
    Mul(Box<TokenTree>, Box<TokenTree>),
    Div(Box<TokenTree>, Box<TokenTree>),
    Exp(Box<TokenTree>, Box<TokenTree>),
    Mod(Box<TokenTree>, Box<TokenTree>),
    Assing(Box<TokenTree>),
    AssingAdd(Box<TokenTree>, Box<TokenTree>),
    AssingSub(Box<TokenTree>, Box<TokenTree>),
    AssingMul(Box<TokenTree>, Box<TokenTree>),
    AssingDiv(Box<TokenTree>, Box<TokenTree>),
    AssingExp(Box<TokenTree>, Box<TokenTree>),
    AssingMod(Box<TokenTree>, Box<TokenTree>),
    PostInc(Box<TokenTree>),
    PostDec(Box<TokenTree>),
    PreInc(Box<TokenTree>),
    PreDec(Box<TokenTree>),
    Fac(Box<TokenTree>),
    Abs(Box<TokenTree>),
    Sqrt(Box<TokenTree>),
    Sin(Box<TokenTree>),
    Cos(Box<TokenTree>),
    Tan(Box<TokenTree>),
    Asin(Box<TokenTree>),
    Acos(Box<TokenTree>),
    Atan(Box<TokenTree>),
}

impl CodeRunner for YololRunner{
    fn compile(path: &str) {
    }

    fn step() {
        todo!()
    }
}