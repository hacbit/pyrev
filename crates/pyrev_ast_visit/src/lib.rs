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

    #[inline]
    pub fn indent(&self) -> String {
        " ".repeat(self.indent)
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
        self.write_newline();
        self.write_back(&self.indent());
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

    /// Write a pass back
    #[inline]
    pub fn add_pass(&mut self) {
        self.write_back("pass");
    }

    /// Add a text back with padding
    ///
    /// You can specified added some text in the left and right
    #[inline]
    pub fn add_with_padding(&mut self, text: &str, left: &str, right: &str) {
        if left.is_empty() {
            self.write_back(left)
        }
        self.write_back(text);
        if right.is_empty() {
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
        self.write_back(&node.operator);
        node.right.and_then(|right| {
            query.get_single(right).map(|res| {
                self.visit(res, query);
            })
        });
    }

    fn visit_function(&mut self, node: &Self::Function, query: &Self::Query) {
        self.write_back(&node.name);
        self.delimit("(", ")", |unparser| {
            for arg in &node.args {
                if let Some(res) = query.get_single(*arg) {
                    unparser.visit(res, query);
                }
            }
        });
    }

    fn visit_function_type(&mut self, node: &Self::Function, query: &Self::Query) {
        if let Some(mut args) = node
            .args
            .iter()
            .map(|id| helper!(query, Some(id), as_ref_fast_variable))
            .collect::<Option<Vec<_>>>()
        {
            args.sort_by(|a, b| a.index.cmp(&b.index));
            if let Some(ret) = args.last()
                && ret.name == "return"
            {
                for arg in args.iter().take(args.len() - 1) {
                    self.write_back(&arg.name);
                    if let Some(anno) = &arg.annotation {
                        self.add_with_padding(anno, " -> ", "")
                    }
                    self.add_comma();
                }
            }
        }
    }

    fn visit_function_arg(&mut self, node: &Self::FunctionArg, _query: &Self::Query) {
        if node.name != "return" {
            self.write_back(&node.name)
        }
        if let Some(anno) = &node.annotation {
            self.add_with_padding(anno, " -> ", "")
        }
    }

    fn visit_function_return(&mut self, node: &Self::FunctionArg, query: &Self::Query) {}

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
}
