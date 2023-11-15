use super::bytecode::Bytecode;
use super::valuetype::{ValueType, ValueTypeVec};
use std::vec::Vec;

/*
一个缓冲区，用来储存解析字节码过程的临时数据
其实就是把Vec<String>封装了一下
*/
type BytecodeBuffer = Vec<String>;

/* 储存一段bytecode指令块，划分依据是源代码的行数
也就是bytecode每段左上的那个数字，那就代表的行数
所以每一个bytecode块就是对应着某一行代码 */
/* example:
1           2 BUILD_LIST               0
            4 LOAD_CONST               0 ((1, 3, 'asf'))
            6 LIST_EXTEND              1
            8 STORE_NAME               0 (a)
*/
/* 第一行的1表示对应到源代码的第一行；
2468是指令偏移，for，if之类的流程控制就是通过这个偏移来确定跳转的位置；
然后是bytecode指令，大部分其实可以根据英文意思看出来功能；
第4列的0010是可以理解为一个参数的代号
第5列就是代表指令的实际参数 */
struct BytecodeBlock {
    script_line_number: u32,
    cmd_offset: Vec<u32>,
    bytecode: Vec<Bytecode>,
    arg: Vec<u32>,
    real_arg: Vec<String>,
    jump_offset: Option<u32>,
} // 这个结构体尽量不要暴露给外部，因为里面的数据结构可能会变

#[allow(unused)]
impl BytecodeBlock {
    fn new() -> BytecodeBlock {
        BytecodeBlock {
            script_line_number: 0,
            cmd_offset: Vec::new(),
            bytecode: Vec::new(),
            arg: Vec::new(),
            real_arg: Vec::new(),
            jump_offset: None,
        }
    }

    /*  把一个bytecode的信息储存到BytecodeBlock中
    参数是一个Vec<String>，每一个String就是一行bytecode
    */
    fn add(&mut self, bytecode_string: &Vec<String>) {
        let mut iter = bytecode_string.iter();
        // 按照空格分割，第一个是行数
        // 只有第一行有行数，后面的行数都是空格
        // 每行只把" (.*)"前的内容用空格分割，后面的内容不分割
        // 长度大于等于42的就是有参数的指令
        let l = iter.next().unwrap();
        let binding = l.split_at(if l.len() >= 42 { 42 } else { l.len() - 1 });
        let mut line = binding.0.split_whitespace();
        let mut real_arg = binding.1.to_string();
        self.script_line_number = line.next().unwrap().parse::<u32>().unwrap();
        loop {
            if let Some(cmd_offset) = line.next() {
                if cmd_offset == ">>" {
                    let cmd_offset = line.next().unwrap();
                }
                let cmd_offset = cmd_offset.parse::<u32>().unwrap_or(0);
                self.cmd_offset.push(cmd_offset);
            }
            if let Some(bytecode) = line.next() {
                self.bytecode.push(Bytecode::get(bytecode));
            }
            if let Some(arg) = line.next() {
                self.arg.push(arg.parse::<u32>().unwrap_or(0));
            }
            // 去除实参最外层的括号
            if real_arg.len() > 3 {
                // 去除前两个字符" ("和最后一个字符")"
                real_arg = real_arg.split_at(2).1.to_string();
                real_arg.pop(); // 去除右侧括号
            }
            self.real_arg.push(real_arg);

            if let Some(l) = iter.next() {
                let binding = l.split_at(if l.len() >= 42 { 42 } else { l.len() });
                line = binding.0.split_whitespace();
                real_arg = binding.1.to_string();
            } else {
                break;
            }
        }
    }

    /* 把BytecodeBlock中的信息转换为python格式
    比如：
    1           2 BUILD_LIST               0
                4 LOAD_CONST               0 ((1, 3, 'asf'))
                6 LIST_EXTEND              1
                8 STORE_NAME               0 (a)
    转换为：
    a = [1, 3, 'asf']
    */

    fn to_python(&mut self) -> String {
        let mut pyscript_line = String::new();
        let mut buffer = BytecodeBuffer::new();
        let mut value_types = ValueTypeVec::new();
        let mut is_for = false;
        let mut is_if = false;
        let mut is_self_calculation = false;
        let mut jump_offset = 0; // 跳转偏移
                                 /* for (i, bcode) in self.bytecode.iter().enumerate() {
                                     let rarg = self.real_arg[i].as_str();
                                     match bcode {
                                         Bytecode::LoadConst
                                         | Bytecode::LoadName
                                         | Bytecode::LoadFast
                                         | Bytecode::LoadGlobal => {
                                             buffer.push(rarg.to_string());
                                         }
                                         Bytecode::StoreName | Bytecode::StoreFast | Bytecode::LoadGlobal => {
                                             if is_for | is_if | is_self_calculation {
                                                 is_for = false;
                                                 let enumer = rarg;
                                                 pyscript_line.replace("i", enumer);
                                             } else {
                                                 let value_type = value_types.pop().unwrap_or(ValueType::None);
                                                 println!("value_type: {:#?}", value_type);
                                                 println!("py_line: {}", pyscript_line);
                                                 self.store(&mut pyscript_line, &mut buffer, rarg, value_type);
                                             }
                                         }
                                         Bytecode::BuildList => value_types.push(ValueType::List),
                                         Bytecode::BuildTuple => value_types.push(ValueType::Tuple),
                                         Bytecode::BuildSet => value_types.push(ValueType::Set),
                                         Bytecode::BuildMap => value_types.push(ValueType::Dict),

                                         Bytecode::BinarySubscr => self.subscr(&mut pyscript_line, &mut buffer),
                                         Bytecode::BinaryOp => self.op(&mut pyscript_line, &mut buffer, rarg, &mut is_self_calculation),

                                         Bytecode::Call => {
                                             self.call(&mut pyscript_line, &mut buffer);
                                         }

                                         Bytecode::ForIter => {
                                             is_for = true;
                                             self.for_iter(&mut pyscript_line, &mut buffer);
                                         }

                                         Bytecode::JumpBackward => {
                                             jump_offset = rarg.trim_start_matches("to ").parse::<u32>().unwrap_or(0);
                                             pyscript_line.push_str(format!("goto {}", jump_offset).as_str());
                                             // 设置jump_offset
                                             if jump_offset != 0 {
                                                 self.jump_offset = Some(jump_offset);
                                             }
                                         }

                                         Bytecode::Nop => {
                                             pyscript_line.push_str("True");
                                         }
                                         _ => {
                                             //
                                         }
                                     }
                                 } */
        pyscript_line
    }

    // todo: 还有很多需要完善的地方
    /* fn load(&self, buffer: &mut BytecodeBuffer, rarg: &str) {
        buffer.push(rarg.to_string());
    } */

    // 调用ValueType的build方法，把buffer中的数据转换为对应的类型的python格式
    // store目前还行，可以先实现load
    fn store(
        &self,
        pyscript: &mut String,
        buffer: &mut BytecodeBuffer,
        rarg: &str,
        value_type: ValueType,
    ) {
        pyscript.push_str(
            value_type
                .build(rarg, buffer.pop().unwrap_or(String::from("")).as_str())
                .as_str(),
        );
    }

    // 下标操作
    fn subscr(&self, pyscript: &mut String, buffer: &mut BytecodeBuffer) {
        let mut key = buffer.pop().unwrap_or(String::from(""));
        let mut name = buffer.pop().unwrap_or(String::from(""));
        buffer.push(format!("{}[{}]", name, key));
    }

    // 操作符
    fn op(
        &self,
        pyscript: &mut String,
        buffer: &mut BytecodeBuffer,
        rarg: &str,
        is_self_calculation: &mut bool,
    ) {
        match rarg {
            "+=" => {
                *is_self_calculation = true;
                let mut right = buffer.pop().unwrap_or(String::from(""));
                let mut left = buffer.pop().unwrap_or(String::from(""));
                pyscript.push_str(format!("{} += {}", left, right).as_str());
            }
            _ => {
                //
            }
        }
    }

    // 调用函数
    fn call(&self, pyscript: &mut String, buffer: &mut BytecodeBuffer) {
        let mut args = buffer.pop().unwrap_or(String::from(""));
        let mut func = buffer.pop().unwrap_or(String::from(""));
        pyscript.push_str(format!("{}({})", func, args).as_str());
    }

    // for循环
    fn for_iter(&self, pyscript: &mut String, buffer: &mut BytecodeBuffer) {
        let mut iter = buffer.pop().unwrap_or(String::from(""));
        pyscript.push_str(format!("for i in {}:", iter).as_str());
    }

    /* // 设置缩进
    unsafe fn set_retractions(&self, pyscript: &mut String) {
        for _ in 0..RETRACTIONS {
            pyscript.push_str("    ");
        }
    } */

    // todo!();
}

// 表示一行python代码，包含行数和代码
#[allow(unused)]
#[derive(Clone, Debug)]
pub struct PyLine {
    pub line: u32,
    pub pyscript: String,
    pub start_offset: u32,
    pub jump_offset: Option<u32>,
    pub retractions: u32,
}

#[allow(unused)]
impl PyLine {
    pub fn new(line: u32, pyscript: String, start_offset: u32, jump_offset: Option<u32>) -> PyLine {
        PyLine {
            line,
            pyscript,
            start_offset,
            jump_offset,
            retractions: 0,
        }
    }
}

// static mut RETRACTIONS: u32 = 0;

// 提供API给外部调用, 用于测试
#[allow(unused)]
pub fn reverse_bytecode(bcs: &Vec<String>) -> PyLine {
    let mut block = BytecodeBlock::new();
    block.add(bcs);
    let pyscript = unsafe { block.to_python() };
    let start_offset = block.cmd_offset[0];
    PyLine::new(
        block.script_line_number,
        pyscript,
        start_offset,
        block.jump_offset,
    )
}
