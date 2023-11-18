use super::binary::{Binary, Unary};
use super::valuetype::ValueType;

// 根据常见的BytecodeType指令做了简单的分类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BytecodeType {
    Load,               // 加载值
    Store,              // 存储值
    Build(ValueType),   // 构建特定类型的值
    Extend,             // 扩展
    Loop,               // 循环
    Jump,               // 跳转
    Function,           // 函数
    Unary(Unary),       // 一元操作
    Binary(Binary),     // 二元操作
    Call,               // 调用函数
    Return,             // 返回值
    Push,               // 压栈
    Pop,                // 出栈
    Import(ImportType), // 导入
    Nop,                //
    None,               // 未知指令
    Other,
}

#[allow(unused)]
impl BytecodeType {
    pub fn get(name: &str) -> BytecodeType {
        if name.contains("FUNCTION") {
            return BytecodeType::Function;
        } else if name.contains("CALL_INTRINSIC") {
            return BytecodeType::Other;
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
            "RETURN" => return BytecodeType::Return,
            "IMPORT" => return BytecodeType::Import(ImportType::get(name_split[1]).unwrap()),
            "JUMP" => return BytecodeType::Jump,
            "LOOP" => return BytecodeType::Loop,
            "NOP" => return BytecodeType::Nop,
            _ => (),
        }

        if name_split[1] == "OP" {
            return BytecodeType::Binary(Binary::get(name_split[0]).unwrap());
        }

        if name.contains("EXTEND") {
            return BytecodeType::Extend;
        }

        BytecodeType::None
    }
}

// Import
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImportType {
    ImportFrom,
    ImportName,
}

#[allow(unused)]
impl ImportType {
    pub fn get(name: &str) -> Option<ImportType> {
        match name {
            "FROM" => Some(ImportType::ImportFrom),
            "NAME" => Some(ImportType::ImportName),
            _ => None,
        }
    }

    pub fn get_expr(&self, expr: &str) -> String {
        match self {
            ImportType::ImportFrom => format!("import {}", expr),
            ImportType::ImportName => format!("from {}", expr),
        }
    }
}
