use regex::Regex;

// 根据常见的BytecodeType指令做了简单的分类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BytecodeType {
    Load,             // 加载值
    Store,            // 存储值
    Build(ValueType), // 构建特定类型的值
    Extend,           // 扩展
    Jump(Jump),       // 跳转
    Function,         // 函数
    Unary(Unary),     // 一元操作
    Binary(Binary),   // 二元操作
    Call,             // 调用函数
    Return,           // 返回值
    Push,             // 压栈
    Pop,              // 出栈
    Import(Import),   // 导入
    Nop,              //
    None,             // 未知指令
    Other,
}

impl BytecodeType {
    pub fn get(name: &str) -> BytecodeType {
        if name.contains("FUNCTION") {
            return BytecodeType::Function;
        } else if name.contains("CALL_INTRINSIC") {
            return BytecodeType::Other;
        } else if name == "LIST_EXTEND" {
            return BytecodeType::Extend;
        }

        let name_split = name.split('_').collect::<Vec<&str>>();
        match name_split[0] {
            "LOAD" => return BytecodeType::Load,
            "STORE" => return BytecodeType::Store,
            "POP" => return BytecodeType::Pop,
            "PUSH" => return BytecodeType::Push,
            "BUILD" => return BytecodeType::Build(ValueType::get(name_split[1]).unwrap()),
            "BINARY" => return BytecodeType::Binary(Binary::get(name_split[1]).unwrap()),
            "UNARY" => return BytecodeType::Unary(Unary::get(name_split[1]).unwrap()),
            "CALL" => return BytecodeType::Call,
            "COPY" => return BytecodeType::Other,
            "RETURN" => return BytecodeType::Return,
            "IMPORT" => return BytecodeType::Import(Import::get(name_split[1]).unwrap()),
            "NOP" => return BytecodeType::Nop,
            _ => (),
        }

        /* let jump_list = ["FOR", "JUMP"];
        // name_split中包含jump_list中任意元素即可
        if jump_list.iter().any(|&x| name_split.contains(&x)) {
            return BytecodeType::Jump;
        } */
        if name_split.contains(&"FOR") {
            return BytecodeType::Jump(Jump::For);
        } else if name_split.contains(&"IF") {
            return BytecodeType::Jump(Jump::If);
        } else if name_split.contains(&"ELSE") {
            return BytecodeType::Jump(Jump::Else);
        } else if name_split.contains(&"WHILE") {
            return BytecodeType::Jump(Jump::While);
        } else if name_split.contains(&"CONTINUE") {
            return BytecodeType::Jump(Jump::Continue);
        } else if name_split.contains(&"BREAK") {
            return BytecodeType::Jump(Jump::Break);
        } else if name_split.contains(&"END") {
            return BytecodeType::Jump(Jump::End);
        }

        if name_split[1] == "OP" {
            return BytecodeType::Binary(Binary::get(name_split[0]).unwrap());
        }

        BytecodeType::None
    }
}

// Import
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Import {
    ImportFrom,
    ImportName,
}

impl Import {
    pub fn get(name: &str) -> Option<Import> {
        match name {
            "FROM" => Some(Import::ImportFrom),
            "NAME" => Some(Import::ImportName),
            _ => None,
        }
    }
}

static mut FOR_STATES: Vec<usize> = Vec::new();
// if true, it means the for expr is not end
static mut FOR_FLAGS: Vec<bool> = Vec::new();
// 状态机， 用于处理for循环， if判断等
pub struct JumpState;

impl JumpState {
    pub fn begin_for(offset: usize) {
        unsafe {
            FOR_STATES.push(offset);
            FOR_FLAGS.push(true);
        }
    }

    pub fn try_end_for(offset: usize) -> bool {
        unsafe {
            if FOR_STATES.is_empty() || FOR_STATES.last().unwrap() != &offset {
                return false;
            }
            FOR_STATES.pop();
            FOR_FLAGS.pop();
            true
        }
    }

    pub fn is_forexpr_not_end() -> bool {
        unsafe { !FOR_FLAGS.is_empty() && *FOR_FLAGS.last().unwrap() }
    }

    pub fn end_forexpr() {
        unsafe {
            if !FOR_FLAGS.is_empty() {
                *FOR_FLAGS.last_mut().unwrap() = false
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Jump {
    For,
    If,
    Else,
    While,
    Continue,
    Break,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/* 一些常见的python类型 */
pub enum ValueType {
    Common, // 一般类型，包括int、float、str、bool等
    List,
    Slice,
    Tuple,
    Set,
    Dict,
    // None,
}

impl ValueType {
    /* 根据不同类型来转换为对应的格式
    其类型将在operator中解析bytecode时确定
    比如解析到BUILD_LIST，就会把类型解释为list */
    pub fn build(&self, value: Option<&str>) -> String {
        match self {
            ValueType::List => {
                if value.is_none() {
                    return String::from("[]");
                }
                let value = Regex::new(r"\((.+)\)")
                    .unwrap()
                    .captures(value.unwrap())
                    .and_then(|cap| cap.get(0))
                    .map_or("", |m| m.as_str());
                format!("[{}]", value)
            }
            ValueType::Slice => value
                .unwrap()
                .split(", ")
                .collect::<Vec<&str>>()
                .join(":")
                .replace("None", ""),
            ValueType::Set => {
                if value.is_none() {
                    return String::from("{}");
                }
                let value = Regex::new(r"\((.+)\)")
                    .unwrap()
                    .captures(value.unwrap())
                    .and_then(|cap| cap.get(0))
                    .map_or("", |m| m.as_str());
                format!("{{{}}}", value)
            }
            ValueType::Tuple => value.unwrap().to_string(),
            ValueType::Dict => {
                if value.is_none() {
                    return String::from("{}");
                }
                let value = Regex::new(r"\((.+)\)")
                    .unwrap()
                    .captures(value.unwrap())
                    .and_then(|cap| cap.get(0))
                    .map_or("", |m| m.as_str());
                format!("{{{}}}", value)
            }
            ValueType::Common => value.unwrap().to_string(),
            // ValueType::None => String::from(""),
        }
    }

    /* pub fn extend(&self, src: &str, etn: &str) -> String {
        if "()[]{}".contains(src) {
            assert!(src.len() == 2);
            format!(
                "{}{}{}",
                src.chars().next().unwrap(),
                Regex::new(r"^\(|\)$|^\[|\]$|^\{|\}$")
                    .unwrap()
                    .replacen(etn, 2, ""),
                src.chars().nth(1).unwrap(),
            )
        } else {
            format!("{} + {}", src, etn)
        }
    } */

    pub fn get(s: &str) -> Option<ValueType> {
        match s.to_lowercase().as_str() {
            "list" => Some(ValueType::List),
            "slice" => Some(ValueType::Slice),
            "tuple" => Some(ValueType::Tuple),
            "set" => Some(ValueType::Set),
            "dict" => Some(ValueType::Dict),
            _ => Some(ValueType::Common),
        }
    }
}

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

impl Unary {
    pub fn get(s: &str) -> Option<Unary> {
        match s {
            "NOT" => Some(Unary::Not),
            "NEGATIVE" => Some(Unary::Neg),
            "INVERT" => Some(Unary::Invert),
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
