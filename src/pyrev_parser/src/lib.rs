//! Parser the opcode to AST

use pyrev_core::opcode::{Opcode, OpcodeInstruction};
use pyrev_ast::*;
use pyrev_query::Map;
use regex::Regex;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn parse(map: &mut Map, opcode_instructions: &[OpcodeInstruction]) -> Result<Vec<QueryId>> {
    let mut expr_ids = Vec::<QueryId>::new();
    let mut offset = 0;
    loop {
        if offset == opcode_instructions.len() {
            break;
        } else if offset > opcode_instructions.len() {
            return Err("Offset out of range".into());
        }

        let instruction = opcode_instructions
            .get(offset)
            .ok_or("[Parse] No instruction found")?;

        let opcode = instruction.opcode();

        match opcode {
            Opcode::LoadConst | Opcode::LoadName | Opcode::LoadGlobal => {
                let base_expr = BaseValue {
                    value: instruction.argval.as_ref().ok_or(format!(
                        "[Load] No argval, offset: {}",
                        instruction.offset
                    ))?
                    .trim_start_matches("NULL + ")
                    .to_string(),
                    start_offset: instruction.offset,
                    end_offset: instruction.offset + 1,
                    ..Default::default()
                };

                let query_id = map.add::<BaseValue>(base_expr.into()).ok_or("[Load] Add base value failed")?;

                expr_ids.push(query_id);
            }
            _ => {}
        }
    }

    Ok(expr_ids)
}