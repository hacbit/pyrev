use super::valuetype::ValueType;

// 根据常见的bytecode指令做了简单的分类
#[derive(Debug)]
pub enum Bytecode {
    Load,             // 加载值
    Store,            // 存储值
    Build(ValueType), // 构建特定类型的值
    Loop,             // 循环
    Jump,             // 跳转
    Function,         // 函数
    Op,               // 操作符
    Call,             // 调用函数
    Return,           // 返回值
    Push,             // 压栈
    Pop,              // 出栈
    Nop,              //
    None,             // 未知指令
}

#[allow(unused)]
impl Bytecode {
    /* 用于将字符串转换为枚举类型，方便后续的匹配 */
    pub fn get(name: &str) -> Bytecode {
        if name.contains(&"FUNCTION") {
            return Bytecode::Function;
        }
        let name_split = name.split("_").collect::<Vec<&str>>();
        match name_split[0] {
            "LOAD" => return Bytecode::Load,
            "STORE" => return Bytecode::Store,
            "POP" => return Bytecode::Pop,
            "PUSH" => return Bytecode::Push,
            "BUILD" => return Bytecode::Build(ValueType::get(name_split[1])),
            "CALL" => return Bytecode::Call,
            "RETURN" => return Bytecode::Return,
            _ => (),
        }

        Bytecode::None
    }
}
