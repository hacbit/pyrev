use std::collections::HashMap;

use pyrev_ast_visit::*;
use pyrev_core::prelude::*;
use pyrev_parser::*;
use pyrev_query::*;

#[cfg(target_os = "windows")]
const NEWLINE: &str = "\r\n";

#[cfg(not(target_os = "windows"))]
const NEWLINE: &str = "\n";

pub fn code_gen(code_map: CodeObjectMap) -> ParseResult<String> {
    let mut query_map = Map::new();
    let mut expr_map = HashMap::new();

    for (mark, code_object) in code_map.iter() {
        let expr = parse(&mut query_map, &code_object)?;

        expr_map.insert(mark.clone(), expr);
    }

    let mut res = vec![];

    for i in expr_map.get("<main>").unwrap().iter() {
        let mut unparser = Unparser::new();
        let code = query_map.get_single(*i).unwrap();
        let code = unparser.visit(code, &query_map);
        res.extend(code);
    }

    Ok(res.join(NEWLINE))
}
