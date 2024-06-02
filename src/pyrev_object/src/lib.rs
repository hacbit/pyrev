use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum PyObject {
    Null,
    None,
    Bool(bool),
    Ellipsis,
    Int(i32),
    Int64(i64),
    Long(PyLong),
    Float(f64),
    Complex(f64, f64),
    String(Vec<u8>),
    AsciiString(String),
    Tuple(Vec<PyObject>),
    List(Vec<PyObject>),
    Dict(Vec<(PyObject, PyObject)>),
    Set(Vec<PyObject>),
    Code(Box<Code>),
}

impl PyObject {
    pub fn is_null(&self) -> bool {
        matches!(self, PyObject::Null)
    }

    pub fn iter(&self) -> impl Iterator<Item = &PyObject> {
        match self {
            PyObject::Tuple(v) | PyObject::List(v) | PyObject::Set(v) => v.iter(),
            _ => panic!("iter() called on non-iterable object"),
        }
    }

    pub fn into_iter(self) -> impl Iterator<Item = PyObject> {
        match self {
            PyObject::Tuple(v) | PyObject::List(v) | PyObject::Set(v) => v.into_iter(),
            _ => panic!("into_iter() called on non-iterable object"),
        }
    }

    pub fn iter_keys(&self) -> impl Iterator<Item = &PyObject> {
        match self {
            PyObject::Dict(v) => v.iter().map(|(k, _)| k),
            _ => panic!("keys() called on non-dict object"),
        }
    }

    pub fn into_iter_keys(self) -> impl Iterator<Item = PyObject> {
        match self {
            PyObject::Dict(v) => v.into_iter().map(|(k, _)| k),
            _ => panic!("keys() called on non-dict object"),
        }
    }

    pub fn iter_values(&self) -> impl Iterator<Item = &PyObject> {
        match self {
            PyObject::Dict(v) => v.iter().map(|(_, v)| v),
            _ => panic!("values() called on non-dict object"),
        }
    }

    pub fn into_iter_values(self) -> impl Iterator<Item = PyObject> {
        match self {
            PyObject::Dict(v) => v.into_iter().map(|(_, v)| v),
            _ => panic!("values() called on non-dict object"),
        }
    }

    pub fn iter_items(&self) -> impl Iterator<Item = (&PyObject, &PyObject)> {
        match self {
            PyObject::Dict(v) => v.iter().map(|(k, v)| (k, v)),
            _ => panic!("items() called on non-dict object"),
        }
    }

    pub fn into_iter_items(self) -> impl Iterator<Item = (PyObject, PyObject)> {
        match self {
            PyObject::Dict(v) => v.into_iter(),
            _ => panic!("items() called on non-dict object"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PyLong {
    pub sign: bool,
    pub size: usize,
    pub value: Vec<u16>,
}

#[derive(Clone, PartialEq)]
pub struct Code {
    pub arg_count: u32,
    pub pos_only_arg_count: u32,
    pub kw_only_arg_count: u32,
    pub stack_size: u32,
    pub flags: u32,
    pub code: PyObject,
    pub consts: PyObject,
    pub names: PyObject,
    pub locals_plus_names: PyObject,
    pub locals_plus_kinds: PyObject,
    pub file_name: PyObject,
    pub name: PyObject,
    pub qual_name: PyObject,
    pub first_line_no: u32,
    pub line_table: PyObject,
    pub exception_table: PyObject,
}

impl fmt::Debug for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Code")
            .field("arg_count", &self.arg_count)
            .field("pos_only_arg_count", &self.pos_only_arg_count)
            .field("kw_only_arg_count", &self.kw_only_arg_count)
            .field("stack_size", &self.stack_size)
            .field("flags", &self.flags)
            .field("code", &self.code)
            .field("consts", &self.consts)
            .field("names", &self.names)
            .field("locals_plus_names", &self.locals_plus_names)
            .field("locals_plus_kinds", &self.locals_plus_kinds)
            .field("file_name", &self.file_name)
            .field("name", &self.name)
            .field("qual_name", &self.qual_name)
            .field("first_line_no", &self.first_line_no)
            .field("line_table", &self.line_table)
            .field("exception_table", &self.exception_table)
            .finish_non_exhaustive()
    }
}
