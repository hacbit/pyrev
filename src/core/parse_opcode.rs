use super::{common::*, opcode::OpcodeInstruction};
use regex::Regex;

pub type ObjectMark = String;
pub type LineNumber = usize;
pub type CodeObject = OrderMap<LineNumber, Vec<OpcodeInstruction>>;
pub type CodeObjectMap = OrderMap<ObjectMark, CodeObject>;

pub trait OpcodeParser {
    fn parse(&self) -> Result<CodeObjectMap>;
}

impl<T> OpcodeParser for T
where
    T: AsRef<str> + 'static,
{
    /// 解析一个字节码文件的内容, 返回一个字节码对象映射表(CodeObjectMap)
    /// ObjectMark 是一个对象的标记(String), 就是字节码里面看到<>包裹的
    /// LineNumber 是一个行号(usize), 就是字节码里每一段左上角的数字
    /// CodeObject 是一个字节码对象, 里面包含了一个对象主体的所有指令
    fn parse(&self) -> Result<CodeObjectMap> {
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
        let mut code_object_map = CodeObjectMap::new();
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
            let instruction = OpcodeInstruction::new(
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
                if code_object.contains_key(&line) {
                    code_object
                        .get_mut(&line)
                        .ok_or("Unknown Line")?
                        .push(instruction);
                } else {
                    code_object.insert(line, vec![instruction]);
                }
            } else {
                code_object
                    .get_mut(&last_line)
                    .ok_or("Unknown Line")?
                    .push(instruction);
            }
        }
        if !code_object_map.contains_key(&this_obj_mark) {
            code_object_map.insert(this_obj_mark, code_object);
        }
        Ok(code_object_map)
    }
}

#[cfg(test)]
mod tests {
    use super::super::opcode::Opcode;
    use super::*;

    #[test]
    fn test_parse_opcode() {
        let bytecode = r#"  1           2 LOAD_CONST               0 ('b')
        4 LOAD_NAME                0 (int)
        6 LOAD_CONST               1 ('return')
        8 LOAD_NAME                0 (int)
       10 BUILD_TUPLE              4
       12 LOAD_CONST               2 (<code object test at 0x0000025C36EDDB80, file "test/def.py", line 1>)
       14 MAKE_FUNCTION            4 (annotations)
       16 STORE_NAME               1 (test)"#.to_string();
        let parsed = bytecode.parse().unwrap();
        //dbg!(parsed);
        //assert!(false);
        assert_eq!(
            parsed.get("<main>").unwrap().get(&1).unwrap(),
            &vec![
                OpcodeInstruction {
                    opcode: Opcode::LoadConst,
                    opname: "LOAD_CONST".to_string(),
                    arg: Some(0,),
                    argval: Some("'b'".to_string(),),
                    offset: 2,
                    starts_line: Some(1,),
                    is_jump_target: false,
                    positions: vec![],
                },
                OpcodeInstruction {
                    opcode: Opcode::LoadName,
                    opname: "LOAD_NAME".to_string(),
                    arg: Some(0,),
                    argval: Some("int".to_string(),),
                    offset: 4,
                    starts_line: None,
                    is_jump_target: false,
                    positions: vec![],
                },
                OpcodeInstruction {
                    opcode: Opcode::LoadConst,
                    opname: "LOAD_CONST".to_string(),
                    arg: Some(1,),
                    argval: Some("'return'".to_string(),),
                    offset: 6,
                    starts_line: None,
                    is_jump_target: false,
                    positions: vec![],
                },
                OpcodeInstruction {
                    opcode: Opcode::LoadName,
                    opname: "LOAD_NAME".to_string(),
                    arg: Some(0,),
                    argval: Some("int".to_string(),),
                    offset: 8,
                    starts_line: None,
                    is_jump_target: false,
                    positions: vec![],
                },
                OpcodeInstruction {
                    opcode: Opcode::BuildTuple,
                    opname: "BUILD_TUPLE".to_string(),
                    arg: Some(4,),
                    argval: None,
                    offset: 10,
                    starts_line: None,
                    is_jump_target: false,
                    positions: vec![],
                },
                OpcodeInstruction {
                    opcode: Opcode::LoadConst,
                    opname: "LOAD_CONST".to_string(),
                    arg: Some(2,),
                    argval: Some(
                        r#"<code object test at 0x0000025C36EDDB80, file "test/def.py", line 1>"#
                            .to_string(),
                    ),
                    offset: 12,
                    starts_line: None,
                    is_jump_target: false,
                    positions: vec![],
                },
                OpcodeInstruction {
                    opcode: Opcode::MakeFunction,
                    opname: "MAKE_FUNCTION".to_string(),
                    arg: Some(4,),
                    argval: Some("annotations".to_string(),),
                    offset: 14,
                    starts_line: None,
                    is_jump_target: false,
                    positions: vec![],
                },
                OpcodeInstruction {
                    opcode: Opcode::StoreName,
                    opname: "STORE_NAME".to_string(),
                    arg: Some(1,),
                    argval: Some("test".to_string(),),
                    offset: 16,
                    starts_line: None,
                    is_jump_target: false,
                    positions: vec![],
                },
            ]
        )
    }
}
