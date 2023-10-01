use super::bytecode::Bytecode;
use super::valuetype::{ValueType, ValueTypeVec};
use std::vec::Vec;

/*
一个缓冲区，用来储存解析字节码过程的临时数据
其实就是把Vec<String>封装了一下
*/
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
        match self.scr.pop() {
            Some(s) => Some(s),
            None => Some(String::from("")),
        }
    }
}

/* 储存一段bytecode指令块，划分依据是源代码的行数
也就是bytecode每段左上的那个数字，那就代表的行数
所以每一个bytecode块就是对应着某一行代码 */
/* example:
1           2 BUILD_LIST               0
            4 LOAD_CONST               0 ((1, 3, 'asf'))
            6 LIST_EXTEND              1
            8 STORE_NAME               0 (a) */
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
        }
    }

    /*  把一个bytecode的信息储存到BytecodeBlock中
    参数是一个Vec<String>，每一个String就是一行bytecode
    */
    fn add(&mut self, bytecode_string: &Vec<String>) {
        let mut iter = bytecode_string.iter();
        // 按照空格分割，第一个是行数
        let mut line: std::str::SplitWhitespace<'_> = iter.next().unwrap().split_whitespace();
        self.script_line_number = line.next().unwrap().parse::<u32>().unwrap();
        loop {
            if let Some(cmd_offset) = line.next() {
                self.cmd_offset.push(cmd_offset.parse::<u32>().unwrap());
            }
            if let Some(bytecode) = line.next() {
                self.bytecode.push(Bytecode::get(bytecode));
            }
            if let Some(arg) = line.next() {
                self.arg.push(arg.parse::<u32>().unwrap());
            }
            // 去除实参最外层的括号
            // 考虑到实参中可能有空格，所以把line后面的所以迭代内容都拼接起来
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

    /* 把BytecodeBlock中的信息转换为python格式
    比如：
    1           2 BUILD_LIST               0
                4 LOAD_CONST               0 ((1, 3, 'asf'))
                6 LIST_EXTEND              1
                8 STORE_NAME               0 (a)
    转换为：
    a = [1, 3, 'asf']
    */
    fn to_python(&self) -> String {
        let mut pyscript_line = String::new();
        let mut buffer = BytecodeBuffer::new();
        let mut value_types = ValueTypeVec::new();
        for (i, bcode) in self.bytecode.iter().enumerate() {
            let rarg = self.real_arg[i].as_str();
            match bcode {
                Bytecode::LoadConst => {
                    self.load(&mut buffer, bcode, rarg);
                }
                Bytecode::StoreName => {
                    self.store(
                        &mut pyscript_line,
                        &mut buffer,
                        rarg,
                        value_types.pop().unwrap(),
                    );
                }
                Bytecode::BuildList => value_types.push(ValueType::List),
                Bytecode::BuildTuple => value_types.push(ValueType::Tuple),
                Bytecode::BuildSet => value_types.push(ValueType::Set),
                Bytecode::BuildMap => value_types.push(ValueType::Dict),
                _ => {
                    //
                }
            }
        }
        pyscript_line
    }

    // todo: 还有很多需要完善的地方
    fn load(&self, buffer: &mut BytecodeBuffer, bytecode: &Bytecode, rarg: &str) {
        match bytecode {
            Bytecode::LoadConst => {
                buffer.push(rarg.to_string());
            }
            _ => {
                // todo!();
            }
        }
    }

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
                .build(rarg, buffer.pop().unwrap().as_str())
                .as_str(),
        );
    }

    // todo!();
}

// 提供API给外部调用, 用于测试
#[allow(unused)]
pub fn test(bcs: &Vec<String>) -> String {
    let mut block = BytecodeBlock::new();
    block.add(bcs);
    block.to_python()
}
