#[derive(Debug)]
struct BytecodeBlock {
    script_line_number: u32,
    cmd_offset: Vec<u32>,
    bytecode: Vec<BytecodeMap>,
    arg: Vec<u32>,
    real_arg: Vec<String>,
}

// 一个缓冲区，用来储存解析字节码过程的临时数据
struct BytecodeBuffer {
    scr: Vec<String>,
}

impl BytecodeBuffer {
    fn new() -> BytecodeBuffer {
        BytecodeBuffer { scr: Vec::new() }
    }

    fn push(&mut self, s: String) {
        self.scr.push(s);
    }

    fn pop(&mut self) -> Option<String> {
        self.scr.pop()
    }
}

impl BytecodeBlock {
    fn new() -> BytecodeBlock {
        BytecodeBlock {
            script_line_number: 0,
            cmd_offset: Vec::new(),
            bytecode: Vec::new(),
            arg: Vec::new(),
            real_arg: Vec::new(),
        }
    }

    fn add(&mut self, bytecode_string: &Vec<String>) {
        let mut iter = bytecode_string.iter();
        let mut line: std::str::SplitWhitespace<'_> = iter.next().unwrap().split_whitespace();
        self.script_line_number = line.next().unwrap().parse::<u32>().unwrap();
        loop {
            if let Some(cmd_offset) = line.next() {
                self.cmd_offset.push(cmd_offset.parse::<u32>().unwrap());
            }
            if let Some(bytecode) = line.next() {
                self.bytecode.push(BytecodeMap::get(bytecode));
            }
            if let Some(arg) = line.next() {
                self.arg.push(arg.parse::<u32>().unwrap());
            }
            let mut real_arg: String = line.collect();
            if real_arg.len() > 0 {
                real_arg.remove(0);
                real_arg.remove(real_arg.len() - 1);
            }
            self.real_arg.push(real_arg);

            if let Some(l) = iter.next() {
                line = l.split_whitespace();
            } else {
                break;
            }
        }
    }
}

#[derive(Debug)]
enum BytecodeMap {
    LoadConst,
    LoadName,
    StoreName,
    LoadFast,
    StoreFast,
    LoadGlobal,
    StoreGlobal,
    BuildList,
    BuildTuple,
    BuildSet,
    BuildMap,
    LoadAttr,
    StoreAttr,
    BuildSlice,
    LoadSubscr,
    StoreSubscr,
    SetupLoop,
    JumpAbsolute,
    PopJumpIfFalse,
    PopJumpIfTrue,
    JumpForward,
    CompareOp,
    MakeFunction,
    ListExtend,
    None,
}

#[allow(unused)]
impl BytecodeMap {
    fn get(name: &str) -> Self {
        match name {
            "LOAD_CONST" => BytecodeMap::LoadConst,
            "LOAD_NAME" => BytecodeMap::LoadName,
            "STORE_NAME" => BytecodeMap::StoreName,
            "LOAD_FAST" => BytecodeMap::LoadFast,
            "STORE_FAST" => BytecodeMap::StoreFast,
            "LOAD_GLOBAL" => BytecodeMap::LoadGlobal,
            "STORE_GLOBAL" => BytecodeMap::StoreGlobal,
            "BUILD_LIST" => BytecodeMap::BuildList,
            "BUILD_TUPLE" => BytecodeMap::BuildTuple,
            "BUILD_SET" => BytecodeMap::BuildSet,
            "BUILD_MAP" => BytecodeMap::BuildMap,
            "LOAD_ATTR" => BytecodeMap::LoadAttr,
            "STORE_ATTR" => BytecodeMap::StoreAttr,
            "BUILD_SLICE" => BytecodeMap::BuildSlice,
            "LOAD_SUBSCR" => BytecodeMap::LoadSubscr,
            "STORE_SUBSCR" => BytecodeMap::StoreSubscr,
            "SETUP_LOOP" => BytecodeMap::SetupLoop,
            "JUMP_ABSOLUTE" => BytecodeMap::JumpAbsolute,
            "POP_JUMP_IF_FALSE" => BytecodeMap::PopJumpIfFalse,
            "POP_JUMP_IF_TRUE" => BytecodeMap::PopJumpIfTrue,
            "JUMP_FORWARD" => BytecodeMap::JumpForward,
            "COMPARE_OP" => BytecodeMap::CompareOp,
            "MAKE_FUNCTION" => BytecodeMap::MakeFunction,
            "LIST_EXTEND" => BytecodeMap::ListExtend,
            _ => BytecodeMap::None,
        }
    }

    fn name(&self) -> &str {
        match self {
            BytecodeMap::LoadConst => "LOAD_CONST",
            BytecodeMap::LoadName => "LOAD_NAME",
            BytecodeMap::StoreName => "STORE_NAME",
            BytecodeMap::LoadFast => "LOAD_FAST",
            BytecodeMap::StoreFast => "STORE_FAST",
            BytecodeMap::LoadGlobal => "LOAD_GLOBAL",
            BytecodeMap::StoreGlobal => "STORE_GLOBAL",
            BytecodeMap::BuildList => "BUILD_LIST",
            BytecodeMap::BuildTuple => "BUILD_TUPLE",
            BytecodeMap::BuildSet => "BUILD_SET",
            BytecodeMap::BuildMap => "BUILD_MAP",
            BytecodeMap::LoadAttr => "LOAD_ATTR",
            BytecodeMap::StoreAttr => "STORE_ATTR",
            BytecodeMap::BuildSlice => "BUILD_SLICE",
            BytecodeMap::LoadSubscr => "LOAD_SUBSCR",
            BytecodeMap::StoreSubscr => "STORE_SUBSCR",
            BytecodeMap::SetupLoop => "SETUP_LOOP",
            BytecodeMap::JumpAbsolute => "JUMP_ABSOLUTE",
            BytecodeMap::PopJumpIfFalse => "POP_JUMP_IF_FALSE",
            BytecodeMap::PopJumpIfTrue => "POP_JUMP_IF_TRUE",
            BytecodeMap::JumpForward => "JUMP_FORWARD",
            BytecodeMap::CompareOp => "COMPARE_OP",
            BytecodeMap::MakeFunction => "MAKE_FUNCTION",
            BytecodeMap::ListExtend => "LIST_EXTEND",
            BytecodeMap::None => "None",
        }
    }
}

#[allow(unused)]
enum ValueType {
    Common,
    List,
    Tuple,
    Set,
    Dict,
    None,
}

#[allow(unused)]
impl ValueType {
    fn build(&self, name: &str, value: &str) -> String {
        let val = value.trim_start_matches("(").trim_end_matches(")");
        match self {
            ValueType::Common => {
                format!("{} = {}", name, val)
            }
            ValueType::List => {
                format!("{} = [{}]", name, val)
            }
            ValueType::Tuple => {
                format!("{} = ({})", name, val)
            }
            ValueType::Set | ValueType::Dict => {
                format!("{} = {{{}}}", name, val)
            }
            ValueType::None => {
                format!("{}", name)
            }
        }
    }
}

#[allow(unused)]
struct ValueTypeVec {
    value_type: Vec<ValueType>,
}

#[allow(unused)]
impl ValueTypeVec {
    fn new() -> ValueTypeVec {
        ValueTypeVec {
            value_type: Vec::new(),
        }
    }

    fn push(&mut self, value_type: ValueType) {
        self.value_type.push(value_type);
    }

    fn pop(&mut self) -> Option<ValueType> {
        match self.value_type.pop() {
            Some(value_type) => Some(value_type),
            None => Some(ValueType::Common),
        }
    }
}

#[allow(unused)]
impl BytecodeBlock {
    fn to_python(&self) -> String {
        let mut pyscript_line = String::new();
        let mut buffer = BytecodeBuffer::new();
        let mut value_types = ValueTypeVec::new();
        for (i, bcode) in self.bytecode.iter().enumerate() {
            let rarg = self.real_arg[i].as_str();
            match bcode {
                BytecodeMap::LoadConst => {
                    self.load(&mut buffer, bcode, rarg);
                }
                BytecodeMap::StoreName => {
                    self.store(
                        &mut pyscript_line,
                        &mut buffer,
                        rarg,
                        value_types.pop().unwrap(),
                    );
                }
                BytecodeMap::BuildList => value_types.push(ValueType::List),
                BytecodeMap::BuildTuple => value_types.push(ValueType::Tuple),
                BytecodeMap::BuildSet => value_types.push(ValueType::Set),
                BytecodeMap::BuildMap => value_types.push(ValueType::Dict),
                _ => {
                    //
                }
            }
        }
        pyscript_line
    }

    fn load(&self, buffer: &mut BytecodeBuffer, bytecode: &BytecodeMap, rarg: &str) {
        match bytecode {
            BytecodeMap::LoadConst => {
                buffer.push(rarg.to_string());
            }
            _ => {
                //
            }
        }
    }

    fn store(
        &self,
        pyscript: &mut String,
        buffer: &mut BytecodeBuffer,
        rarg: &str,
        value_type: ValueType,
    ) {
        pyscript.push_str(
            value_type
                .build(rarg, buffer.pop().unwrap().as_str())
                .as_str(),
        );
    }
}

use std::vec::Vec;

pub fn test(bcs: &Vec<String>) -> String {
    let mut block = BytecodeBlock::new();
    block.add(bcs);
    block.to_python()
}
