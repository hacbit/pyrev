// 处理op操作， 比如加减乘除等
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OP {
    Add,   // +
    Sub,   // -
    Mul,   // *
    Div,   // /
    Mod,   // %
    Xor,   // ^
    Or,    // |
    And,   // &
    Not,   // ~
    Eq,    // ==   Equal
    Ne,    // !=   Not Equal
    Gt,    // >    Greater than
    Lt,    // <    Less than
    Ge,    // >=   Greater than or equal to
    Le,    // <=   Less than or equal to
    AddEq, // +=
    SubEq, // -=
    MulEq, // *=
    DivEq, // /=
    ModEq, // %=
    XorEq, // ^=
    OrEq,  // |=
    AndEq, // &=
    NotEq, // ~=
}

#[allow(unused)]
impl OP {
    pub fn from_str(s: &str) -> Option<OP> {
        match s {
            "+" => Some(OP::Add),
            "-" => Some(OP::Sub),
            "*" => Some(OP::Mul),
            "/" => Some(OP::Div),
            "%" => Some(OP::Mod),
            "^" => Some(OP::Xor),
            "|" => Some(OP::Or),
            "&" => Some(OP::And),
            "~" => Some(OP::Not),
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
            "~=" => Some(OP::NotEq),
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
            OP::Not => "~",
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
            OP::NotEq => "~=",
        }
    }

    pub fn get_expr(&self, left: &str, right: &str) -> String {
        format!("{} {} {}", left, self.to_str(), right)
    }
}
