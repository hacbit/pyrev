//! A visitor for the Python AST.

#![feature(let_chains)]

use pyrev_ast::*;
use pyrev_query::*;

pub trait PyNodeVisitor {
    type Node;
    type Constant;
    type Container;
    type Function;
    type FunctionArg;
    type Callable;
    type UnaryOp;
    type BinaryOp;
    type Assign;
    type Alias;
    type Class;
    type Attribute;
    type Output;
    type Query;

    /// Visit a node.
    fn visit(&mut self, node: &Self::Node, query: &Self::Query) -> Self::Output;

    fn visit_constant(&mut self, node: &Self::Constant, query: &Self::Query);

    fn visit_list(&mut self, node: &Self::Container, query: &Self::Query);

    fn visit_list_comp(&mut self, node: &Self::Function, query: &Self::Query);

    fn visit_tuple(&mut self, node: &Self::Container, query: &Self::Query);

    fn visit_generator(&mut self, node: &Self::Function, query: &Self::Query);

    fn visit_set(&mut self, node: &Self::Container, query: &Self::Query);

    fn visit_set_comp(&mut self, node: &Self::Function, query: &Self::Query);

    fn visit_dict(&mut self, node: &Self::Container, query: &Self::Query);

    fn visit_dict_comp(&mut self, node: &Self::Function, query: &Self::Query);

    fn visit_unary_op(&mut self, node: &Self::UnaryOp, query: &Self::Query);

    fn visit_binary_op(&mut self, node: &Self::BinaryOp, query: &Self::Query);

    fn visit_function(&mut self, node: &Self::Function, query: &Self::Query);

    fn visit_function_type(&mut self, node: &Self::Function, query: &Self::Query);

    fn visit_function_arg(&mut self, node: &Self::FunctionArg, query: &Self::Query);

    fn visit_function_return(&mut self, node: &Self::FunctionArg, query: &Self::Query);

    fn visit_call(&mut self, node: &Self::Callable, query: &Self::Query);

    fn visit_assign(&mut self, node: &Self::Assign, query: &Self::Query);

    fn visit_alias(&mut self, node: &Self::Alias, query: &Self::Query);

    fn visit_class(&mut self, node: &Self::Class, query: &Self::Query);

    fn visit_class_docstring(&mut self, node: &Self::Constant, query: &Self::Query);

    fn visit_attribute(&mut self, node: &Self::Attribute, query: &Self::Query);
}

pub struct Unparser {
    indent: usize,
    output: Vec<String>,
}

impl Unparser {
    #[inline]
    pub fn new() -> Self {
        Self {
            indent: 4,
            output: vec![],
        }
    }

    /// Write a newline with a indent.
    #[inline]
    pub fn add_indent(&mut self) {
        self.write_newline();
        self.write_back(&" ".repeat(self.indent));
    }

    /// Write a text to the last line of the output.
    /// Don't add a new line.
    ///
    /// If the output is empty, a new line is added.
    #[inline]
    pub fn write_back(&mut self, text: &str) {
        if let Some(last) = self.output.last_mut() {
            last.push_str(text);
        } else {
            self.output.push(text.to_string());
        }
    }

    /// Add a new line.
    #[inline]
    pub fn write_newline(&mut self) {
        self.output.push("".to_string());
    }

    /// Add a new line and indent with the a given text.
    #[inline]
    pub fn write_newline_indent_with(&mut self, text: &str) {
        self.add_indent();
        self.write_back(text);
    }

    /// Delimit the output with the given left and right text.
    #[inline]
    pub fn delimit(&mut self, left: &str, right: &str, f: impl FnOnce(&mut Self)) {
        self.write_back(left);
        f(self);
        self.write_back(right);
    }

    /// Write a comma back
    #[inline]
    pub fn add_comma(&mut self) {
        self.write_back(", ");
    }

    /// Add a text back with padding
    ///
    /// You can specified added some text in the left and right
    #[inline]
    pub fn add_with_padding(&mut self, text: &str, left: &str, right: &str) {
        if !left.is_empty() {
            self.write_back(left)
        }
        self.write_back(text);
        if !right.is_empty() {
            self.write_back(right)
        }
    }
}

impl PyNodeVisitor for Unparser {
    type Node = ExpressionEnum;
    type Constant = BaseValue;
    type Container = Container;
    type Function = Function;
    type FunctionArg = FastVariable;
    type Callable = Call;
    type UnaryOp = UnaryOperation;
    type BinaryOp = BinaryOperation;
    type Assign = Assign;
    type Alias = Alias;
    type Class = Class;
    type Attribute = Attribute;
    type Output = Vec<String>;
    type Query = Map;

    fn visit(&mut self, node: &Self::Node, query: &Self::Query) -> Self::Output {
        match node {
            ExpressionEnum::BaseValue(base) => {
                self.visit_constant(base, query);
            }
            ExpressionEnum::Container(container) => match container.container_type {
                ContainerType::List => self.visit_list(container, query),
                ContainerType::Tuple => self.visit_tuple(container, query),
                ContainerType::Set => self.visit_set(container, query),
                ContainerType::Dict => self.visit_dict(container, query),
            },
            ExpressionEnum::Function(function) => match function.name.as_str() {
                "<listcomp>" => self.visit_list_comp(function, query),
                "<setcomp>" => self.visit_set_comp(function, query),
                "<dictcomp>" => self.visit_dict_comp(function, query),
                name => {
                    if name.contains(&['<', '>']) {
                        self.visit_generator(function, query)
                    } else {
                        self.visit_function(function, query)
                    }
                }
            },
            ExpressionEnum::Call(call) => self.visit_call(call, query),
            ExpressionEnum::BinaryOperation(bin_op) => self.visit_binary_op(bin_op, query),
            ExpressionEnum::UnaryOperation(unary_op) => self.visit_unary_op(unary_op, query),
            ExpressionEnum::Assign(assign) => self.visit_assign(assign, query),
            ExpressionEnum::Alias(alias) => self.visit_alias(alias, query),
            ExpressionEnum::Class(class) => self.visit_class(class, query),
            ExpressionEnum::Attribute(attr) => self.visit_attribute(attr, query),
            _ => {}
        }

        self.output.clone()
    }

    fn visit_constant(&mut self, node: &Self::Constant, _query: &Self::Query) {
        self.write_back(&node.value);
    }

    fn visit_list(&mut self, node: &Self::Container, query: &Self::Query) {
        debug_assert!(node.container_type == ContainerType::List);

        self.delimit("[", "]", |unparser| {
            for element in &node.values {
                if let Some(res) = query.get_single(*element) {
                    unparser.visit(res, query);
                }
            }
        })
    }

    fn visit_tuple(&mut self, node: &Self::Container, query: &Self::Query) {
        debug_assert!(node.container_type == ContainerType::Tuple);

        self.delimit("(", ")", |unparser| {
            for element in &node.values {
                if let Some(res) = query.get_single(*element) {
                    unparser.visit(res, query);
                }
            }
        })
    }

    fn visit_set(&mut self, node: &Self::Container, query: &Self::Query) {
        debug_assert!(node.container_type == ContainerType::Set);

        self.delimit("{", "}", |unparser| {
            for element in &node.values {
                if let Some(res) = query.get_single(*element) {
                    unparser.visit(res, query);
                }
            }
        })
    }

    fn visit_dict(&mut self, node: &Self::Container, query: &Self::Query) {
        debug_assert!(node.container_type == ContainerType::Dict);

        self.delimit("{", "}", |unparser| {
            for (i, v) in node.values.iter().enumerate() {
                if i % 2 == 0 {
                    if let Some(res) = query.get_single(*v) {
                        unparser.visit(res, query);
                    }
                } else {
                    unparser.write_back(": ");
                    if let Some(res) = query.get_single(*v) {
                        unparser.visit(res, query);
                    }
                }
            }
        })
    }

    fn visit_list_comp(&mut self, node: &Self::Function, _query: &Self::Query) {
        debug_assert!(node.name == "<listcomp>");

        self.delimit("[", "]", |_unparser| todo!())
    }

    fn visit_set_comp(&mut self, node: &Self::Function, _query: &Self::Query) {
        debug_assert!(node.name == "<setcomp>");

        self.delimit("{", "}", |_unparser| todo!())
    }

    fn visit_dict_comp(&mut self, node: &Self::Function, _query: &Self::Query) {
        debug_assert!(node.name == "<dictcomp>");

        self.delimit("{", "}", |_unparser| todo!())
    }

    fn visit_generator(&mut self, _node: &Self::Function, _query: &Self::Query) {
        todo!()
    }

    fn visit_unary_op(&mut self, node: &Self::UnaryOp, query: &Self::Query) {
        let op = match node.unary_type {
            UnaryType::Not => "not ",
            UnaryType::Invert => "~",
            UnaryType::Positive => "+",
            UnaryType::Negative => "-",
        };
        self.write_back(op);
        node.target.and_then(|target| {
            query.get_single(target).map(|res| {
                self.visit(res, query);
            })
        });
    }

    fn visit_binary_op(&mut self, node: &Self::BinaryOp, query: &Self::Query) {
        node.left.and_then(|left| {
            query.get_single(left).map(|res| {
                self.visit(res, query);
            })
        });
        self.add_with_padding(&node.operator, " ", " ");
        node.right.and_then(|right| {
            query.get_single(right).map(|res| {
                self.visit(res, query);
            })
        });
    }

    fn visit_function(&mut self, node: &Self::Function, query: &Self::Query) {
        self.write_back("def ");
        self.write_back(&node.name);
        self.visit_function_type(node, query);
        self.write_back(":");

        for item_id in node.bodys.iter() {
            if let Some(item) = query.get_single(*item_id) {
                self.add_indent();
                self.visit(item, query);
            } else {
                // get item failed
                self.write_newline();
            }
        }

        if node.bodys.is_empty() {
            self.write_newline_indent_with("pass");
        }

        self.write_newline();
    }

    fn visit_function_type(&mut self, node: &Self::Function, query: &Self::Query) {
        self.delimit("(", ")", |unparser| {
            for (i, arg_id) in node.args.iter().enumerate() {
                if let Some(arg) = helper!(query, Some(arg_id), as_ref_fast_variable) {
                    unparser.visit_function_arg(arg, query);
                    if i < node.args.len() - 1 {
                        unparser.add_comma();
                    }
                }
            }
        });

        if let Some(ret) = helper!(query, node.ret.as_ref(), as_ref_fast_variable) {
            self.visit_function_return(ret, query)
        }
    }

    fn visit_function_arg(&mut self, node: &Self::FunctionArg, _query: &Self::Query) {
        self.write_back(&node.name);
        if let Some(anno) = &node.annotation {
            self.add_with_padding(anno, ": ", "")
        }
    }

    fn visit_function_return(&mut self, node: &Self::FunctionArg, _query: &Self::Query) {
        debug_assert_eq!(node.name, "return");

        if let Some(anno) = &node.annotation {
            self.add_with_padding(anno, " -> ", "")
        }
    }

    fn visit_call(&mut self, node: &Self::Callable, query: &Self::Query) {
        node.func.and_then(|func| {
            query.get_single(func).map(|res| {
                self.visit(res, query);
            })
        });
        self.delimit("(", ")", |unparser| {
            for (i, arg) in node.args.iter().enumerate() {
                if let Some(res) = query.get_single(*arg) {
                    unparser.visit(res, query);
                }
                if i < node.args.len() - 1 {
                    unparser.add_comma();
                }
            }
        })
    }

    fn visit_assign(&mut self, node: &Self::Assign, query: &Self::Query) {
        node.target.and_then(|target| {
            query.get_single(target).map(|res| {
                self.visit(res, query);
            })
        });

        self.add_with_padding(&node.operator, " ", " ");

        node.value.and_then(|value| {
            query.get_single(value).map(|res| {
                self.visit(res, query);
            })
        });
    }

    fn visit_alias(&mut self, node: &Self::Alias, query: &Self::Query) {
        node.target.and_then(|target| {
            query.get_single(target).map(|res| {
                self.visit(res, query);
            })
        });

        self.write_back(" as ");

        node.alias.and_then(|value| {
            query.get_single(value).map(|res| {
                self.visit(res, query);
            })
        });
    }

    fn visit_class(&mut self, node: &Self::Class, query: &Self::Query) {
        self.write_back("class ");
        self.write_back(&node.name);
        self.write_back(":");

        for item_id in node.members.iter() {
            if let Some(item) = query.get_single(*item_id) {
                self.add_indent();
                if let Some(member) = item.as_ref_assign() {
                    if let Some(target) = helper!(query, member.target.as_ref(), as_ref_base_value)
                    {
                        match target.value.as_str() {
                            "__doc__" => {
                                member.value.and_then(|docstring| {
                                    query.get_single(docstring).map(|res| {
                                        res.as_ref_base_value().map(|bv| {
                                            self.visit_class_docstring(bv, query);
                                        })
                                    })
                                });
                            }
                            // ignore __module__ and __qualname__
                            "__module__" | "__qualname__" => {}
                            _ => {
                                self.visit(item, query);
                            }
                        }
                    } else {
                        self.visit(item, query);
                    }
                }
            } else {
                // get item failed
                self.add_indent();
                self.write_back("# failed to get item");
            }

            self.write_newline();
            self.write_newline();
        }

        if node.members.is_empty() {
            self.write_newline_indent_with("pass");
        }

        self.write_newline();
    }

    fn visit_class_docstring(&mut self, node: &Self::Constant, _query: &Self::Query) {
        let docstring = &node.value;
        let docstring = docstring.trim_matches('\'');
        
        self.write_newline_indent_with("\"\"\"");

        for line in docstring.lines().filter(|l| !l.trim().is_empty()) {
            self.write_newline_indent_with(line);
        }

        self.write_newline_indent_with("\"\"\"");
    }

    fn visit_attribute(&mut self, node: &Self::Attribute, query: &Self::Query) {
        node.parent.and_then(|parent| {
            query.get_single(parent).map(|res| {
                self.visit(res, query);
            })
        });

        self.write_back(".");

        node.attr.and_then(|attr| {
            query.get_single(attr).map(|res| {
                self.visit(res, query);
            })
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static mut MAP: Option<Map> = None;

    macro_rules! get_map {
        () => {
            unsafe {
                if MAP.is_none() {
                    MAP = Some(Map::new());
                }
                MAP.as_ref().unwrap()
            }
        };
        (mut) => {
            unsafe {
                if MAP.is_none() {
                    MAP = Some(Map::new());
                }
                MAP.as_mut().unwrap()
            }
        };
    }

    macro_rules! maybe_have {
        ($id:ident) => {
            Some(stringify!($id).to_string())
        };
        ($id:ident, $block:block) => {
            Some($block)
        };
        () => {
            None
        };
    }

    macro_rules! function_expr {
        (
            def $name:ident(
                $($arg:ident $(: $anno:ident)?),*
            ) $( -> $ret:ident)?:
                pass
        ) => {
            {
                let map = get_map!(mut);
                let args = vec![
                    $(map.add::<FastVariable>(ExpressionEnum::FastVariable(FastVariable {
                        name: stringify!($arg).to_string(),
                        annotation: maybe_have!($($anno)?),
                        ..Default::default()
                    })).unwrap(),)*
                ];
                let ret = map.add::<FastVariable>(ExpressionEnum::FastVariable(FastVariable {
                    name: "return".to_string(),
                    annotation: maybe_have!($($ret)?),
                    ..Default::default()
                }));
                let bodys = vec![];
                map.add::<Function>(ExpressionEnum::Function(Function {
                    name: stringify!($name).to_string(),
                    args,
                    ret,
                    bodys,
                    ..Default::default()
                })).unwrap()
            }
        };
    }

    macro_rules! unary_expr {
        ($op:tt $right:expr) => {{
            let map = get_map!(mut);
            let right = map.add::<BaseValue>(ExpressionEnum::BaseValue(BaseValue {
                value: $right.to_string(),
                ..Default::default()
            }));
            map.add::<UnaryOperation>(ExpressionEnum::UnaryOperation(UnaryOperation {
                target: right,
                unary_type: match stringify!($op) {
                    "not" => UnaryType::Not,
                    "~" => UnaryType::Invert,
                    "+" => UnaryType::Positive,
                    "-" => UnaryType::Negative,
                    _ => unreachable!(),
                },
                ..Default::default()
            }))
        }};
    }

    macro_rules! binary_expr {
        ($left:ident $op:tt $right:expr) => {{
            let map = get_map!(mut);
            let left = map.add::<BaseValue>(ExpressionEnum::BaseValue(BaseValue {
                value: stringify!($left).to_string(),
                ..Default::default()
            }));
            let right = map.add::<BaseValue>(ExpressionEnum::BaseValue(BaseValue {
                value: stringify!($right).to_string(),
                ..Default::default()
            }));
            map.add::<BinaryOperation>(ExpressionEnum::BinaryOperation(BinaryOperation {
                left,
                operator: stringify!($op).to_string(),
                right,
                ..Default::default()
            }))
        }};
    }

    macro_rules! attr_expr {
        ($parent:ident . $attr:ident) => {{
            let map = get_map!(mut);
            let parent = map.add::<BaseValue>(ExpressionEnum::BaseValue(BaseValue {
                value: stringify!($parent).to_string(),
                ..Default::default()
            }));
            let attr = map.add::<BaseValue>(ExpressionEnum::BaseValue(BaseValue {
                value: stringify!($attr).to_string(),
                ..Default::default()
            }));
            map.add::<Attribute>(ExpressionEnum::Attribute(Attribute {
                parent,
                attr,
                ..Default::default()
            }))
        }};
    }

    macro_rules! expr {
        (
            def $name:ident(
                $($arg:ident $(: $anno:ident)?),*
            ) $( -> $ret:ident)?:
                pass
        ) => {
            function_expr!(
                def $name(
                    $($arg $(: $anno)?),*
                ) $( -> $ret)?:
                    pass
            )
        };
        ($op:tt $right:expr) => {
            unary_expr!($op $right)
        };
        ($left:ident $op:tt $right:expr) => {
            binary_expr!($left $op $right)
        };
        ($e:expr) => {
            if let Some(map) = unsafe { MAP.as_mut() } {
                map.add::<BaseValue>(ExpressionEnum::BaseValue(BaseValue {
                    value: $e.to_string(),
                    ..Default::default()
                }))
            } else {
                None
            }
        }
    }

    #[allow(dead_code)]
    fn output_with_line(output: Vec<String>) {
        for (i, line) in output.iter().enumerate() {
            println!("{:03}| {}", i, line);
        }
    }

    #[test]
    fn test_binary_op() {
        let bv = expr!(A = 1).unwrap();
        let bv = get_map!().get_single(bv).unwrap();

        let mut unparser = Unparser::new();

        let res = unparser.visit(&bv, get_map!());

        assert_eq!(res, vec!["A = 1".to_string()]);
    }

    #[test]
    fn test_unary_op() {
        let bv = expr!(not 1).unwrap();
        let bv = get_map!().get_single(bv).unwrap();

        let mut unparser = Unparser::new();

        let res = unparser.visit(&bv, get_map!());

        assert_eq!(res, vec!["not A".to_string()]);
    }

    #[test]
    fn test_function() {
        let func = expr! {
            def add(a, b, c: int) -> int:
                pass
        };

        let func = get_map!().get_single(func).unwrap();

        let mut unparser = Unparser::new();

        let res = unparser.visit(&func, get_map!());

        let res = res.join("\n");

        assert_eq!(res, "def add(a, b, c: int) -> int:\n    pass\n".to_string());
    }

    #[test]
    fn test_attr() {
        let attr = attr_expr!(BBB.a).unwrap();

        let attr = get_map!().get_single(attr).unwrap();

        let mut unparser = Unparser::new();

        let res = unparser.visit(&attr, get_map!());

        assert_eq!(res, vec!["BBB.a".to_string()]);
    }
}
