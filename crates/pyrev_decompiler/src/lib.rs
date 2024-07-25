use std::collections::HashMap;

use pyrev_ast::*;
use pyrev_ast_visit::*;
use pyrev_core::prelude::*;
use pyrev_parser::*;
use pyrev_query::*;

#[cfg(target_os = "windows")]
const NEWLINE: &str = "\r\n";

#[cfg(not(target_os = "windows"))]
const NEWLINE: &str = "\n";

pub fn parse_opcode(s: impl AsRef<str> + 'static) -> Result<CodeObjectMap> {
    s.parse_opcode()
}

pub fn code_gen(code_map: CodeObjectMap) -> ParseResult<String> {
    let mut map = Map::new();
    let mut expr_map = HashMap::new();

    for (mark, code_object) in code_map.iter() {
        let expr = parse(&mut map, &code_object)?;

        expr_map.insert(mark.clone(), expr);
    }

    let mut unparser = Unparser::new();

    let mut res = vec![];

    for i in expr_map.get("<main>").unwrap().iter() {
        let code = map.get_single(*i).unwrap();
        let code = unparser.visit(code, &map);
        res.extend(code);
    }

    Ok(res.join(NEWLINE))
}
