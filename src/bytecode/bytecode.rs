/*
先列了一些比较常用的bytecode，后续根据需要再补充
一些复杂的指令可以先跳过
*/
#[derive(Debug)]
pub enum Bytecode {
    /* 加载值 */
    LoadConst,
    LoadName,
    LoadFast,
    LoadGlobal,
    LoadAttr,
    LoadSubscr,
    /* 存储值 */
    StoreName,
    StoreFast,
    StoreGlobal,
    StoreAttr,
    StoreSubscr,
    /* 构建特殊类型 */
    BuildList,
    BuildTuple,
    BuildSet,
    BuildMap, // 这个是字典（dict），不是map
    BuildSlice,
    /* 主要是跳转循环这些 */
    SetupLoop,
    JumpAbsolute,
    PopJumpIfFalse,
    PopJumpIfTrue,
    JumpForward,
    JumpBackward,
    CompareOp,
    MakeFunction,
    ListExtend,
    BinarySubscr,
    BinaryAdd,
    BinaryMultiply,
    BinaryDivide,
    BinaryModulo,
    Call,
    GetIter,
    ForIter,
    None,
}

#[allow(unused)]
impl Bytecode {
    /* 用于将字符串转换为枚举类型，方便后续的匹配 */
    pub fn get(name: &str) -> Self {
        match name {
            "LOAD_CONST" => Bytecode::LoadConst,
            "LOAD_NAME" => Bytecode::LoadName,
            "LOAD_FAST" => Bytecode::LoadFast,
            "LOAD_GLOBAL" => Bytecode::LoadGlobal,
            "LOAD_ATTR" => Bytecode::LoadAttr,
            "LOAD_SUBSCR" => Bytecode::LoadSubscr,
            "STORE_NAME" => Bytecode::StoreName,
            "STORE_FAST" => Bytecode::StoreFast,
            "STORE_GLOBAL" => Bytecode::StoreGlobal,
            "STORE_ATTR" => Bytecode::StoreAttr,
            "STORE_SUBSCR" => Bytecode::StoreSubscr,
            "BUILD_LIST" => Bytecode::BuildList,
            "BUILD_TUPLE" => Bytecode::BuildTuple,
            "BUILD_SET" => Bytecode::BuildSet,
            "BUILD_MAP" => Bytecode::BuildMap,
            "BUILD_SLICE" => Bytecode::BuildSlice,
            "SETUP_LOOP" => Bytecode::SetupLoop,
            "JUMP_ABSOLUTE" => Bytecode::JumpAbsolute,
            "POP_JUMP_IF_FALSE" => Bytecode::PopJumpIfFalse,
            "POP_JUMP_IF_TRUE" => Bytecode::PopJumpIfTrue,
            "JUMP_FORWARD" => Bytecode::JumpForward,
            "JUMP_BACKWARD" => Bytecode::JumpBackward,
            "COMPARE_OP" => Bytecode::CompareOp,
            "MAKE_FUNCTION" => Bytecode::MakeFunction,
            "LIST_EXTEND" => Bytecode::ListExtend,
            "BINARY_SUBSCR" => Bytecode::BinarySubscr,
            "BINARY_ADD" => Bytecode::BinaryAdd,
            "BINARY_MULTIPLY" => Bytecode::BinaryMultiply,
            "BINARY_DIVIDE" => Bytecode::BinaryDivide,
            "BINARY_MODULO" => Bytecode::BinaryModulo,
            "CALL" => Bytecode::Call,
            "GET_ITER" => Bytecode::GetIter,
            "FOR_ITER" => Bytecode::ForIter,
            _ => Bytecode::None,
        }
    }

    // 直接用枚举类型匹配性能更好
    // 现在暂时不需要这个函数
    /* fn name(&self) -> &str {
        match self {
            Bytecode::LoadConst => "LOAD_CONST",
            Bytecode::LoadName => "LOAD_NAME",
            Bytecode::StoreName => "STORE_NAME",
            Bytecode::LoadFast => "LOAD_FAST",
            Bytecode::StoreFast => "STORE_FAST",
            Bytecode::LoadGlobal => "LOAD_GLOBAL",
            Bytecode::StoreGlobal => "STORE_GLOBAL",
            Bytecode::BuildList => "BUILD_LIST",
            Bytecode::BuildTuple => "BUILD_TUPLE",
            Bytecode::BuildSet => "BUILD_SET",
            Bytecode::BuildMap => "BUILD_MAP",
            Bytecode::LoadAttr => "LOAD_ATTR",
            Bytecode::StoreAttr => "STORE_ATTR",
            Bytecode::BuildSlice => "BUILD_SLICE",
            Bytecode::LoadSubscr => "LOAD_SUBSCR",
            Bytecode::StoreSubscr => "STORE_SUBSCR",
            Bytecode::SetupLoop => "SETUP_LOOP",
            Bytecode::JumpAbsolute => "JUMP_ABSOLUTE",
            Bytecode::PopJumpIfFalse => "POP_JUMP_IF_FALSE",
            Bytecode::PopJumpIfTrue => "POP_JUMP_IF_TRUE",
            Bytecode::JumpForward => "JUMP_FORWARD",
            Bytecode::CompareOp => "COMPARE_OP",
            Bytecode::MakeFunction => "MAKE_FUNCTION",
            Bytecode::ListExtend => "LIST_EXTEND",
            Bytecode::None => "None",
        }
    } */
}
