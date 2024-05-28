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
    pub use super::ast::{ExprParser, get_trace};
    pub use super::common::{IStream, OStream, OrderMap, Result, TraceBack, Local, Colorize};
    pub use super::decompile::{DecompiledCode, Decompiler};
    pub use super::opcode::{Opcode, OpcodeInstruction};
    pub use super::parse_opcode::OpcodeParser;
    pub use crate::{error, info, warn};
}
