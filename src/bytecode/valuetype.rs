#[allow(unused)]
#[derive(Debug)]
/* 一些常见的python类型 */
pub enum ValueType {
    Common, // 一般类型，包括int、float、str、bool等
    List,
    Tuple,
    Set,
    Dict,
    None,
}

#[allow(unused)]
impl ValueType {
    /* 根据不同类型来转换为对应的格式
    其类型将在operator中解析bytecode时确定
    比如解析到BUILD_LIST，就会把类型解释为list */
    pub fn build(&self, name: &str, value: &str) -> String {
        // 去除字符串两端的括号
        /* 注意！
        这可能会破坏某些数据，比如value是((1, 2), (2, 3))
        去除括号后就会变成 1, 2), (2, 3 */
        let val = match value.chars().next().unwrap_or(' ') {
            '(' | '[' | '{' => value[1..value.len() - 1].to_string(),
            _ => value.to_string(),
        };
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

// 把Vec<ValueType>抽象一层，方便后续的操作
#[allow(unused)]
pub struct ValueTypeVec {
    value_type: Vec<ValueType>,
}

#[allow(unused)]
impl ValueTypeVec {
    pub fn new() -> ValueTypeVec {
        ValueTypeVec {
            value_type: Vec::new(),
        }
    }

    pub fn push(&mut self, value_type: ValueType) {
        self.value_type.push(value_type);
    }

    /* 改了一下pop，当pop为空值是（None），不会直接返回Option::None,
    而是返回Option<ValueType>, 保证unwrap()一定会返回相同类型，即ValueType */
    pub fn pop(&mut self) -> Option<ValueType> {
        match self.value_type.pop() {
            Some(value_type) => Some(value_type),
            None => Some(ValueType::None), // 暂时设置为ValueType::Common，后续可能会改为ValueType::None
        }
    }
}
