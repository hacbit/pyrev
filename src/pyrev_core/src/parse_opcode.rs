use super::{common::*, opcode::OpcodeInstruction};
use regex::Regex;

pub type ObjectMark = String;
pub type LineNumber = usize;
//pub type CodeObject = OrderMap<LineNumber, Vec<OpcodeInstruction>>;
pub type CodeObject = Vec<OpcodeInstruction>;
pub type CodeObjectMap = OrderMap<ObjectMark, CodeObject>;

pub trait OpcodeParser {
    fn parse_opcode(&self) -> Result<CodeObjectMap>;
}

impl<T> OpcodeParser for T
where
    T: AsRef<str> + 'static,
{
    /// 解析一个字节码文件的内容, 返回一个字节码对象映射表(CodeObjectMap)
    /// ObjectMark 是一个对象的标记(String), 就是字节码里面看到<>包裹的
    /// LineNumber 是一个行号(usize), 就是字节码里每一段左上角的数字
    /// CodeObject 是一个字节码对象, 里面包含了一个对象主体的所有指令
    fn parse_opcode(&self) -> Result<CodeObjectMap> {
        let reg = Regex::new(
            r#"(?s)(?x)
            (Disassembly\ of\ (?P<mark>[\S\ ]+):\s+)?      # mark  (optional)
            (?P<line>\d+)?          # line  (optional)
            ([\ >]+)?
            (?P<off>\d+)            # offset
            [\ ]
            (?P<bc>[A-Z0-9_]+)      # bytecode
            ([\ ]+)?
            (?P<a>\d+)?             # arg   (optional)
            [\ ]?
            (\((?P<ra>[\S\ ]+)\))?       # real arg  (optional)
            "#,
        )?;
        let mut last_line = 0;
        let mut this_obj_mark = "<main>".to_string();
        let mut code_object = CodeObject::new();
        let mut code_object_map = CodeObjectMap::default();
        for cap in reg.captures_iter(self.as_ref()) {
            let mark = cap.name("mark").map_or("", |m| m.as_str());
            let line = cap.name("line").map_or("", |m| m.as_str());
            let off = cap.name("off").map_or("", |m| m.as_str());
            let bc = cap.name("bc").map_or("", |m| m.as_str());
            let a = cap.name("a").map_or("", |m| m.as_str());
            let ra = cap.name("ra").map_or("", |m| m.as_str());
            // turn to next mark
            //println!("mark: {}", mark);
            if !mark.is_empty() {
                code_object_map.insert(this_obj_mark.clone(), code_object);
                this_obj_mark = mark.to_string();
                code_object = CodeObject::new();
            }
            let mut instruction = OpcodeInstruction::new(
                bc,
                a.parse::<usize>().ok(),
                if ra.is_empty() {
                    None
                } else {
                    Some(ra.to_string())
                },
                off.parse::<usize>()?,
                line.parse::<LineNumber>().ok(),
            );
            if let Ok(line) = line.parse::<LineNumber>() {
                last_line = line;
            } else {
                instruction.starts_line = Some(last_line);
            }
            code_object.push(instruction);
        }
        if !code_object_map.contains_key(&this_obj_mark) {
            code_object_map.insert(this_obj_mark, code_object);
        }
        Ok(code_object_map)
    }
}
