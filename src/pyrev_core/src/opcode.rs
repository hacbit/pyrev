use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpcodeInstruction {
    pub opcode: RefCell<Opcode>,
    pub opname: String,
    pub arg: Option<usize>,
    pub argval: Option<String>,
    pub offset: usize,
    pub starts_line: Option<usize>,
    // If jump here, is_jump_target is true
    pub is_jump_target: bool,
    // Version >= 3.11, save the begin and end of the source code
    pub positions: Vec<usize>,
}

impl OpcodeInstruction {
    pub fn new(
        opname: &str,
        arg: Option<usize>,
        argval: Option<String>,
        offset: usize,
        starts_line: Option<usize>,
    ) -> Self {
        let opcode = match opname {
            "NOP" => Opcode::Nop,
            "POP_TOP" => Opcode::PopTop,
            "END_FOR" => Opcode::EndFor,
            "END_SEND" => Opcode::EndSend,
            "COPY" => Opcode::Copy,
            "SWAP" => Opcode::Swap,
            "UNARY_NEGATIVE" => Opcode::UnaryNegative,
            "UNARY_NOT" => Opcode::UnaryNot,
            "UNARY_INVERT" => Opcode::UnaryInvert,
            "GET_ITER" => Opcode::GetIter,
            "GET_YIELD_FROM_ITER" => Opcode::GetYieldFromIter,
            "BINARY_OP" => Opcode::BinaryOp,
            "BINARY_SUBSCR" => Opcode::BinarySubscr,
            "STORE_SUBSCR" => Opcode::StoreSubscr,
            "DELETE_SUBSCR" => Opcode::DeleteSubscr,
            "BINARY_SLICE" => Opcode::BinarySlice,
            "STORE_SLICE" => Opcode::StoreSlice,
            "GET_AWAITABLE" => Opcode::GetAwaitable,
            "GET_AITER" => Opcode::GetAiter,
            "GET_ANEXT" => Opcode::GetAnext,
            "END_ASYNC_FOR" => Opcode::EndAsyncFor,
            "CLEANUP_THROW" => Opcode::CleanupThrow,
            "BEFORE_ASYNC_WITH" => Opcode::BeforeAsyncWith,
            "SET_ADD" => Opcode::SetAdd,
            "LIST_APPEND" => Opcode::ListAppend,
            "MAP_ADD" => Opcode::MapAdd,
            "RETURN_VALUE" => Opcode::ReturnValue,
            "RETURN_CONST" => Opcode::ReturnConst,
            "YIELD_VALUE" => Opcode::YieldValue,
            "SETUP_ANNOTATIONS" => Opcode::SetupAnnotations,
            "POP_EXCEPT" => Opcode::PopExcept,
            "RERAISE" => Opcode::Reraise,
            "PUSH_EXC_INFO" => Opcode::PushExcInfo,
            "CHECK_EXC_MATCH" => Opcode::CheckExcMatch,
            "CHECK_EG_MATCH" => Opcode::CheckEgMatch,
            "WITH_EXCEPT_START" => Opcode::WithExceptStart,
            "LOAD_ASSERTION_ERROR" => Opcode::LoadAssertionError,
            "LOAD_BUILD_CLASS" => Opcode::LoadBuildClass,
            "BEFORE_WITH" => Opcode::BeforeWith,
            "GET_LEN" => Opcode::GetLen,
            "MATCH_MAPPING" => Opcode::MatchMapping,
            "MATCH_SEQUENCE" => Opcode::MatchSequence,
            "MATCH_KEYS" => Opcode::MatchKeys,
            "STORE_NAME" => Opcode::StoreName,
            "DELETE_NAME" => Opcode::DeleteName,
            "UNPACK_SEQUENCE" => Opcode::UnpackSequence,
            "UNPACK_EX" => Opcode::UnpackEx,
            "STORE_ATTR" => Opcode::StoreAttr,
            "DELETE_ATTR" => Opcode::DeleteAttr,
            "STORE_GLOBAL" => Opcode::StoreGlobal,
            "DELETE_GLOBAL" => Opcode::DeleteGlobal,
            "LOAD_CONST" => Opcode::LoadConst,
            "LOAD_NAME" => Opcode::LoadName,
            "LOAD_LOCALS" => Opcode::LoadLocals,
            "LOAD_FROM_DICT_OR_GLOBALS" => Opcode::LoadFromDictOrGlobals,
            "LOAD_METHOD" => Opcode::LoadMethod,
            "BUILD_TUPLE" => Opcode::BuildTuple,
            "BUILD_LIST" => Opcode::BuildList,
            "BUILD_SET" => Opcode::BuildSet,
            "BUILD_MAP" => Opcode::BuildMap,
            "BUILD_CONST_KEY_MAP" => Opcode::BuildConstKeyMap,
            "BUILD_STRING" => Opcode::BuildString,
            "LIST_EXTEND" => Opcode::ListExtend,
            "SET_UPDATE" => Opcode::SetUpdate,
            "DICT_UPDATE" => Opcode::DictUpdate,
            "DICT_MERGE" => Opcode::DictMerge,
            "LOAD_ATTR" => Opcode::LoadAttr,
            "LOAD_SUPER_ATTR" => Opcode::LoadSuperAttr,
            "COMPARE_OP" => Opcode::CompareOp,
            "IS_OP" => Opcode::IsOp,
            "CONTAINS_OP" => Opcode::ContainsOp,
            "IMPORT_NAME" => Opcode::ImportName,
            "IMPORT_FROM" => Opcode::ImportFrom,
            "JUMP_FORWARD" => Opcode::JumpForward,
            "JUMP_BACKWARD" => Opcode::JumpBackward,
            "JUMP_BACKWARD_NO_INTERRUPT" => Opcode::JumpBackwardNoInterrupt,
            "POP_JUMP_IF_TRUE" | "POP_JUMP_FORWARD_IF_TRUE" | "POP_JUMP_BACKWARD_IF_TRUE" => {
                Opcode::PopJumpIfTrue
            }
            "POP_JUMP_IF_FALSE" | "POP_JUMP_FORWARD_IF_FALSE" | "POP_JUMP_BACKWARD_IF_FALSE" => {
                Opcode::PopJumpIfFalse
            }
            "POP_JUMP_IF_NOT_NONE"
            | "POP_JUMP_FORWARD_IF_NOT_NONE"
            | "POP_JUMP_BACKWARD_IF_NOT_NONE" => Opcode::PopJumpIfNotNone,
            "POP_JUMP_IF_NONE" | "POP_JUMP_FORWARD_IF_NONE" | "POP_JUMP_BACKAWARD_IF_NONE" => {
                Opcode::PopJumpIfNone
            }
            "FOR_ITER" => Opcode::ForIter,
            "LOAD_GLOBAL" => Opcode::LoadGlobal,
            "LOAD_FAST" => Opcode::LoadFast,
            "LOAD_FAST_CHECK" => Opcode::LoadFastCheck,
            "LOAD_FAST_AND_CLEAR" => Opcode::LoadFastAndClear,
            "STORE_FAST" => Opcode::StoreFast,
            "DELETE_FAST" => Opcode::DeleteFast,
            "MAKE_CELL" => Opcode::MakeCell,
            "LOAD_CLOSURE" => Opcode::LoadClosure,
            "LOAD_DEREF" => Opcode::LoadDeref,
            "LOAD_FROM_DICT_OR_DEREF" => Opcode::LoadFromDictOrDeref,
            "STORE_DEREF" => Opcode::StoreDeref,
            "DELETE_DEREF" => Opcode::DeleteDeref,
            "COPY_FREE_VARS" => Opcode::CopyFreeVars,
            "RAISE_VARARGS" => Opcode::RaiseVarargs,
            "CALL" | "CALL_FUNCTION" => Opcode::Call,
            "CALL_FUNCTION_EX" => Opcode::CallFunctionEx,
            "PUSH_NULL" => Opcode::PushNull,
            "KW_NAMES" => Opcode::KwNames,
            "MAKE_FUNCTION" => Opcode::MakeFunction,
            "BUILD_SLICE" => Opcode::BuildSlice,
            "EXTENDED_ARG" => Opcode::ExtendedArg,
            "FORMAT_VALUE" => Opcode::FormatValue,
            "MATCH_CLASS" => Opcode::MatchClass,
            "RESUME" => Opcode::Resume,
            "RETURN_GENERATOR" => Opcode::ReturnGenerator,
            "SEND" => Opcode::Send,
            "CALL_INTRINSIC1" => Opcode::CallIntrinsic1,
            "CALL_INTRINSIC2" => Opcode::CallIntrinsic2,
            _ => Opcode::None,
        };
        Self {
            opcode: opcode.into(),
            opname: opname.to_string(),
            arg,
            argval,
            offset,
            starts_line,
            is_jump_target: false,
            positions: vec![],
        }
    }

    pub fn opcode(&self) -> Opcode {
        self.opcode.borrow().clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opcode {
    None, // It means the opcode is unknown or not implemented
    Nop,
    PopTop,
    // 3.12 added
    EndFor,
    // 3.12 added
    EndSend,
    // 3.11 added
    Copy, // copy the i element to the top of the stack
    // 3.11 added
    Swap, // swap the i element with the top of the stack
    // 3.11 added
    Cache,

    // Unary operations
    UnaryPositive,
    UnaryNegative,
    UnaryNot,
    UnaryInvert,
    GetIter,
    // 3.5 added
    GetYieldFromIter,

    // Binary and Inplace operations
    // 3.11 added
    BinaryOp,
    BinarySubscr,
    StoreSubscr,
    DeleteSubscr,
    // 3.12 added
    BinarySlice,
    // 3.12 added
    StoreSlice,

    // Coroutine opcodes
    // 3.5 added, 3.11 changed
    GetAwaitable, // before 3.11, not have argument
    // 3.5 added, 3.7 changed
    GetAiter, // no return awaitable object from __aiter__ after 3.7
    // 3.5 added
    GetAnext,
    // 3.8 added, 3.11 changed
    EndAsyncFor,
    // 3.12
    CleanupThrow,
    // 3.5
    BeforeAsyncWith,

    SetAdd,
    ListAppend,
    // 3.8 changed
    MapAdd, // After 3.8, key is stack[-2], value is stack[-1]
    ReturnValue,
    // 3.12 added
    ReturnConst,
    // 3.11 changed, 3.12 changed
    YieldValue, //
    // 3.6 added
    SetupAnnotations,
    // 3.11 changed
    PopExcept,
    // 3.9 added, 3.11 changed
    Reraise,
    // 3.11 added
    PushExcInfo,
    // 3.11 added
    CheckExcMatch,
    // 3.11 added
    CheckEgMatch,
    // 3.9 added, 3.11 changed
    WithExceptStart,
    // 3.9 added
    LoadAssertionError,
    LoadBuildClass,
    // 3.11 added
    BeforeWith,
    // 3.10 added
    GetLen,
    // 3.10 added
    MatchMapping,
    // 3.10 added
    MatchSequence,
    // 3.10 added, 3.11 changed
    MatchKeys,
    StoreName,
    DeleteName,
    UnpackSequence,
    UnpackEx,
    StoreAttr,
    DeleteAttr,
    StoreGlobal,
    DeleteGlobal,
    LoadConst,
    LoadName,
    // 3.12 added
    LoadLocals,
    // 3.12 added
    LoadFromDictOrGlobals,
    LoadMethod,
    BuildTuple,
    BuildList,
    BuildSet,
    BuildMap,
    BuildConstKeyMap,
    // 3.6 added
    BuildString,
    // 3.9 added
    ListExtend,
    // 3.9 added
    SetUpdate,
    // 3.9 added
    DictUpdate,
    // 3.9 added
    DictMerge,
    LoadAttr,
    // 3.12 added
    LoadSuperAttr,
    CompareOp,
    // 3.9 added
    IsOp,
    // 3.9 added
    ContainsOp,
    ImportName,
    ImportFrom,
    JumpForward,
    // 3.11 added
    JumpBackward,
    // 3.11 added
    JumpBackwardNoInterrupt,
    PopJumpIfTrue,
    PopJumpIfFalse,
    PopJumpIfNotNone,
    PopJumpIfNone,
    ForIter,
    LoadGlobal,
    LoadFast,
    LoadFastCheck,
    // 3.12 added
    LoadFastAndClear,
    StoreFast,
    DeleteFast,
    // 3.11 added
    MakeCell,
    LoadClosure,
    LoadDeref,
    LoadFromDictOrDeref,
    StoreDeref,
    DeleteDeref,
    CopyFreeVars,
    RaiseVarargs,
    // 3.11 added
    Call,
    CallFunctionEx,
    // 3.11 added
    PushNull,
    // 3.11 added
    KwNames,
    MakeFunction,
    BuildSlice,
    ExtendedArg,
    // 3.6 added
    FormatValue,
    // 3.10 added
    MatchClass,
    // 3.11 added
    Resume,
    // 3.11 added
    ReturnGenerator,
    // 3.11 added
    Send,
    // HaveArgument,
    // 3.12 added
    CallIntrinsic1,
    // 3.12 added
    CallIntrinsic2,
    // others
    // ...
}
