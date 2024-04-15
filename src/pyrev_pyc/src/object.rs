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

pub trait Object {}

impl Object for PyObject {}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PyLong {
    pub sign: bool,
    pub size: usize,
    pub value: Vec<u16>,
}

impl Object for PyLong {}

#[derive(Debug, Clone, PartialEq)]
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
