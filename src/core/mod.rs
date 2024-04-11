// 包括了opcode和opcodeInstruction的定义
pub mod opcode;

// 从文本中解析出opcode
pub mod parse_opcode;

// 一些io等与bytecode关联不大的操作
pub mod common;

// 从opcode反编译出python代码
pub mod decompile;

// 分析opcode生成ast
pub mod ast;

#[allow(unused)]
pub mod prelude {
    pub use super::ast::*;
    pub use super::common::*;
    pub use super::decompile::{DecompiledCode, Decompiler};
    pub use super::opcode::*;
    pub use super::parse_opcode::*;
    pub use crate::{error, info, warn};
}
