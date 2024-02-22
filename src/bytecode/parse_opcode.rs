use super::opcode::OpcodeInstruction;

use regex::Regex;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait ParseOpcode {
    fn parse(&self) -> Result<HashMap<usize, Vec<OpcodeInstruction>>>;
}

impl<T> ParseOpcode for T
where
    T: AsRef<str> + 'static,
{
    fn parse(&self) -> Result<HashMap<usize, Vec<OpcodeInstruction>>> {
        let reg = Regex::new(
            r#"(?x)
            (?P<line>\d+)?          # line  (optional)
            ([\s>]+)?
            (?P<off>\d+)            # offset
            [\s+]
            (?P<bc>[A-Z0-9_]+)      # bytecode
            ([\ ]+)?
            (?P<a>\d+)?             # arg   (optional)
            [\s+]?
            (\((?P<ra>.+)\))?       # real arg  (optional)
            "#,
        )
        .unwrap();
        let mut last_line = 0;
        let mut opcode_instructions: HashMap<usize, Vec<OpcodeInstruction>> = HashMap::new();
        for cap in reg.captures_iter(self.as_ref()) {
            let line = cap.name("line").map_or("", |m| m.as_str());
            let off = cap.name("off").map_or("", |m| m.as_str());
            let bc = cap.name("bc").map_or("", |m| m.as_str());
            let a = cap.name("a").map_or("", |m| m.as_str());
            let ra = cap.name("ra").map_or("", |m| m.as_str());
            let instruction = OpcodeInstruction::new(
                bc,
                a.parse::<usize>().ok(),
                if ra.is_empty() {
                    None
                } else {
                    Some(ra.to_string())
                },
                off.parse::<usize>()?,
                line.parse::<usize>().ok(),
            );
            if let Ok(line) = line.parse::<usize>() {
                last_line = line;
                if opcode_instructions.contains_key(&line) {
                    opcode_instructions
                        .get_mut(&line)
                        .ok_or("Unknown Line")?
                        .push(instruction);
                } else {
                    opcode_instructions.insert(line, vec![instruction]);
                }
            } else {
                opcode_instructions
                    .get_mut(&last_line)
                    .ok_or("Unknown Line")?
                    .push(instruction);
            }
        }
        Ok(opcode_instructions)
    }
}

#[cfg(test)]
mod tests {
    use super::super::opcode::Opcode;
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_opcode() {
        let bytecode = fs::read_to_string("test/import.txt").unwrap();
        let parsed = bytecode.parse().unwrap();
        assert_eq!(
            parsed.get(&1).unwrap().get(0).unwrap(),
            &OpcodeInstruction {
                opcode: Opcode::LoadConst,
                opname: "LOAD_CONST".to_string(),
                arg: Some(0,),
                argval: Some("0".to_string(),),
                offset: 2,
                starts_line: Some(1,),
                is_jump_target: false,
                positions: vec![],
            },
        )
    }
}
