use crate::ast::Tree;

peg::parser! {
    pub grammar yolol_parser() for str{
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
            ":" s:$(b:alphanumeric()*) {Tree::GlobalVariable(s.to_string())}
            / !("if" / "end"/ "goto" ) s:$((a:alpha() b:alphanumeric()*)) {Tree::LocalVariable(s.to_string())}
        rule litteral() -> Tree =
            "-" d:$(digit()*) "." r:$(digit()+) {let d : i64 = ("-".to_string()+d).parse().unwrap();let r: i64 = match r.len() {1 => r.parse::<i64>().unwrap() * 100,2 => r.parse::<i64>().unwrap() * 10,_ => r[0..r.len().min(3)].parse().unwrap(),};Tree::Numerical((d * 1000).saturating_sub(r))}
            / "-" d:$(digit()+) {let d : i64 = ("-".to_string()+d).parse().unwrap();Tree::Numerical(d * 1000)}
            / d:$(digit()*) "." r:$(digit()+) {let d : i64 = d.parse().unwrap();let r: i64 = match r.len() {1 => r.parse::<i64>().unwrap() * 100,2 => r.parse::<i64>().unwrap() * 10,_ => r[0..r.len().min(3)].parse().unwrap(),};Tree::Numerical((d * 1000).saturating_add(r))}
            / d:$(digit()+) {let d : i64 = d.parse().unwrap();Tree::Numerical(d * 1000)}
            / "\"" s:$([^ '"']*) "\"" {Tree::String(s.to_string())}
    }
}
