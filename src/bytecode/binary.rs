#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Binary {
    Op,
    Subscr,
    Slice,
    Compare,
    In,
    Is,
}

impl Binary {
    pub fn get(s: &str) -> Option<Binary> {
        match s.to_lowercase().as_str() {
            "op" => Some(Binary::Op),
            "subscr" => Some(Binary::Subscr),
            "slice" => Some(Binary::Slice),
            "compare" => Some(Binary::Compare),
            "contains" => Some(Binary::In),
            "is" => Some(Binary::Is),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unary {
    Not,    // ! / not
    Neg,    // -
    Invert, // ~
}

#[allow(unused)]
impl Unary {
    pub fn get(s: &str) -> Option<Unary> {
        match s.to_lowercase().as_str() {
            "not" => Some(Unary::Not),
            "negative" => Some(Unary::Neg),
            "invert" => Some(Unary::Invert),
            _ => None,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Unary::Not => "not ",
            Unary::Neg => "-",
            Unary::Invert => "~",
        }
    }

    pub fn get_expr(&self, expr: &str) -> String {
        format!("{}{}", self.to_str(), expr)
    }
}

// 处理op操作， 比如加减乘除等
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OP {
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Mod,      // %
    Xor,      // ^
    Or,       // |
    And,      // &
    FloorDiv, // //
    Pow,      // **
    LShift,   // <<
    RShift,   // >>
    Eq,       // ==   Equal
    Ne,       // !=   Not Equal
    Gt,       // >    Greater than
    Lt,       // <    Less than
    Ge,       // >=   Greater than or equal to
    Le,       // <=   Less than or equal to
    AddEq,    // +=
    SubEq,    // -=
    MulEq,    // *=
    DivEq,    // /=
    ModEq,    // %=
    XorEq,    // ^=
    OrEq,     // |=
    AndEq,    // &=
    InvEq,    // ~=
    LShiftEq, // <<=
    RShiftEq, // >>=
}

#[allow(unused)]
impl OP {
    pub fn get(s: &str) -> Option<OP> {
        match s {
            "+" => Some(OP::Add),
            "-" => Some(OP::Sub),
            "*" => Some(OP::Mul),
            "/" => Some(OP::Div),
            "%" => Some(OP::Mod),
            "^" => Some(OP::Xor),
            "|" => Some(OP::Or),
            "&" => Some(OP::And),
            "//" => Some(OP::FloorDiv),
            "**" => Some(OP::Pow),
            "<<" => Some(OP::LShift),
            ">>" => Some(OP::RShift),
            "==" => Some(OP::Eq),
            "!=" => Some(OP::Ne),
            ">" => Some(OP::Gt),
            "<" => Some(OP::Lt),
            ">=" => Some(OP::Ge),
            "<=" => Some(OP::Le),
            "+=" => Some(OP::AddEq),
            "-=" => Some(OP::SubEq),
            "*=" => Some(OP::MulEq),
            "/=" => Some(OP::DivEq),
            "%=" => Some(OP::ModEq),
            "^=" => Some(OP::XorEq),
            "|=" => Some(OP::OrEq),
            "&=" => Some(OP::AndEq),
            "~=" => Some(OP::InvEq),
            "<<=" => Some(OP::LShiftEq),
            ">>=" => Some(OP::RShiftEq),
            _ => None,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            OP::Add => "+",
            OP::Sub => "-",
            OP::Mul => "*",
            OP::Div => "/",
            OP::Mod => "%",
            OP::Xor => "^",
            OP::Or => "|",
            OP::And => "&",
            OP::FloorDiv => "//",
            OP::Pow => "**",
            OP::LShift => "<<",
            OP::RShift => ">>",
            OP::Eq => "==",
            OP::Ne => "!=",
            OP::Gt => ">",
            OP::Lt => "<",
            OP::Ge => ">=",
            OP::Le => "<=",
            OP::AddEq => "+=",
            OP::SubEq => "-=",
            OP::MulEq => "*=",
            OP::DivEq => "/=",
            OP::ModEq => "%=",
            OP::XorEq => "^=",
            OP::OrEq => "|=",
            OP::AndEq => "&=",
            OP::InvEq => "~=",
            OP::LShiftEq => "<<=",
            OP::RShiftEq => ">>=",
        }
    }

    pub fn get_expr(&self, left: &str, right: &str) -> String {
        format!("{} {} {}", left, self.to_str(), right)
    }
}
