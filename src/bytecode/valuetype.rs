use regex::Regex;

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    pub fn build(&self, value: Option<&str>) -> String {
        match self {
            ValueType::List => {
                if value.is_none() {
                    return String::from("[]");
                }
                let value = Regex::new(r"\(.+\))")
                    .unwrap()
                    .captures(value.unwrap())
                    .and_then(|cap| cap.get(0))
                    .map_or("", |m| m.as_str());
                format!("[{}]", value)
            }
            ValueType::Set => {
                if value.is_none() {
                    return String::from("{}");
                }
                let value = Regex::new(r"\(.+\))")
                    .unwrap()
                    .captures(value.unwrap())
                    .and_then(|cap| cap.get(0))
                    .map_or("", |m| m.as_str());
                format!("{{{}}}", value)
            }
            ValueType::Tuple => value.unwrap().to_string(),
            ValueType::Dict => {
                if value.is_none() {
                    return String::from("{}");
                }
                let value = Regex::new(r"\(.+\))")
                    .unwrap()
                    .captures(value.unwrap())
                    .and_then(|cap| cap.get(0))
                    .map_or("", |m| m.as_str());
                format!("{{{}}}", value)
            }
            ValueType::Common => value.unwrap().to_string(),
            ValueType::None => String::from(""),
        }
    }

    pub fn extend(&self, src: &str, etn: &str) -> String {
        if "()[]{}".contains(src) {
            assert!(src.len() == 2);
            format!(
                "{}{}{}",
                src.chars().next().unwrap(),
                Regex::new(r"^\(|\)$|^\[|\]$|^\{|\}$")
                    .unwrap()
                    .replacen(etn, 2, ""),
                src.chars().nth(1).unwrap(),
            )
        } else {
            format!("{} + {}", src, etn)
        }
    }

    pub fn get(s: &str) -> ValueType {
        match s.to_lowercase().as_str() {
            "list" => ValueType::List,
            "tuple" => ValueType::Tuple,
            "set" => ValueType::Set,
            "dict" => ValueType::Dict,
            _ => ValueType::Common,
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
