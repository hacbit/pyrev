use crate::object::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// pub const MAX_MARSHAL_STACK_DEPTH: usize = 2000;

#[repr(u8)]
#[derive(Debug)]
pub enum Type {
    Null = b'0',
    None = b'N',
    False = b'F',
    True = b'T',
    StopIter = b'S',
    Ellipsis = b'.',
    Int = b'i',
    Int64 = b'I',
    Float = b'f',
    BinaryFloat = b'g',
    Complex = b'x',
    BinaryComplex = b'y',
    Long = b'l',
    String = b's',
    Interned = b't',
    Ref = b'r',
    Tuple = b'(',
    List = b'[',
    Dict = b'{',
    Code = b'c',
    Unicode = b'u',
    Unknown = b'?',
    Set = b'<',
    FrozenSet = b'>',
    FlagRef = b'\x80',
    Ascii = b'a',
    AsciiInterned = b'A',
    SmallTuple = b')',
    ShortAscii = b'z',
    ShortAsciiInterned = b'Z',
}

impl Type {
    fn try_from(byte: u8) -> Result<Self> {
        match byte {
            b'0' => Ok(Type::Null),
            b'N' => Ok(Type::None),
            b'F' => Ok(Type::False),
            b'T' => Ok(Type::True),
            b'S' => Ok(Type::StopIter),
            b'.' => Ok(Type::Ellipsis),
            b'i' => Ok(Type::Int),
            b'I' => Ok(Type::Int64),
            b'f' => Ok(Type::Float),
            b'g' => Ok(Type::BinaryFloat),
            b'x' => Ok(Type::Complex),
            b'y' => Ok(Type::BinaryComplex),
            b'l' => Ok(Type::Long),
            b's' => Ok(Type::String),
            b't' => Ok(Type::Interned),
            b'r' => Ok(Type::Ref),
            b'(' => Ok(Type::Tuple),
            b'[' => Ok(Type::List),
            b'{' => Ok(Type::Dict),
            b'c' => Ok(Type::Code),
            b'u' => Ok(Type::Unicode),
            b'?' => Ok(Type::Unknown),
            b'<' => Ok(Type::Set),
            b'>' => Ok(Type::FrozenSet),
            b'\x80' => Ok(Type::FlagRef),
            b'a' => Ok(Type::Ascii),
            b'A' => Ok(Type::AsciiInterned),
            b')' => Ok(Type::SmallTuple),
            b'z' => Ok(Type::ShortAscii),
            b'Z' => Ok(Type::ShortAsciiInterned),
            _ => Err(format!("Unknown type: 0x{:x}", byte).into()),
        }
    }
}

pub struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
    refs: Vec<PyObject>,
    level: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            refs: Vec::new(),
            level: 0,
        }
    }

    pub fn read_bytes(&mut self, n: usize) -> &'a [u8] {
        if n > self.data.len() - self.pos {
            eprintln!(
                "Error: range end index {} out of range for slice of length {}",
                self.pos + n,
                self.data.len()
            );
            println!("Code: {:?}", self.refs);
            std::process::exit(1);
        }
        let bytes = &self.data[self.pos..self.pos + n];
        self.pos += n;
        bytes
    }

    pub fn read_byte(&mut self) -> u8 {
        self.read_bytes(1)[0]
    }

    pub fn read_short(&mut self) -> u16 {
        let bytes = self.read_bytes(2);
        u16::from_le_bytes([bytes[0], bytes[1]])
    }

    pub fn read_int(&mut self) -> i32 {
        let bytes = self.read_bytes(4);
        i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    pub fn read_long(&mut self) -> i64 {
        let bytes = self.read_bytes(8);
        i64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }

    pub fn read_pylong(&mut self) -> PyLong {
        let n = self.read_int();
        let size = n.unsigned_abs() as usize;
        let mut value = Vec::with_capacity(size);
        for _ in 0..size {
            value.push(self.read_short());
        }
        let sign = n < 0;
        PyLong { sign, size, value }
    }

    pub fn read_float_from_str(&mut self) -> f64 {
        let length = self.read_byte();
        let buf = self.read_bytes(length as usize);
        std::str::from_utf8(buf).unwrap().parse().unwrap()
    }

    pub fn read_float_from_bin(&mut self) -> f64 {
        let buf = self.read_bytes(8);
        f64::from_bits(u64::from_le_bytes([
            buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
        ]))
    }

    pub fn read_ref_reserve(&mut self, flag: u8) -> usize {
        if flag > 0 {
            let idx = self.refs.len();
            self.refs.push(PyObject::None);
            idx
        } else {
            0
        }
    }

    pub fn insert_ref(&mut self, obj: PyObject, idx: usize, flag: u8) -> PyObject {
        if flag > 0 {
            if let Some(r) = self.refs.get_mut(idx) {
                *r = obj.clone();
            } else {
                eprintln!("Index out of range: {}", idx)
            }
        }
        obj
    }

    pub fn read_ref(&mut self, obj: PyObject, flag: u8) -> PyObject {
        #[cfg(debug_assertions)]
        {
            assert!(flag & Type::FlagRef as u8 != 0);
        }
        self.refs.push(obj.clone());
        obj
    }

    pub fn r_ref(&mut self, obj: PyObject, flag: u8) -> PyObject {
        if flag > 0 {
            self.read_ref(obj.clone(), flag)
        } else {
            obj
        }
    }

    pub fn read_object(&mut self) -> PyObject {
        let old_level = self.level;
        match self._r_object() {
            Ok(obj) => {
                self.level = old_level;
                obj
            }
            Err(e) => {
                self.level = old_level;
                eprintln!("Error: {}", e);
                PyObject::None
            }
        }
    }

    fn _r_object(&mut self) -> Result<PyObject> {
        let code_byte = self.read_byte();
        let flag = code_byte & Type::FlagRef as u8;
        let co_type = code_byte & !(Type::FlagRef as u8);
        let co_type = Type::try_from(co_type)?;
        #[cfg(debug_assertions)]
        {
            println!(
                "{} {:?} {:?} {:?}",
                "  ".repeat(self.level),
                code_byte,
                flag,
                co_type
            );
        }
        self.level += 1;

        match co_type {
            Type::Null => Ok(PyObject::Null),
            Type::None => Ok(PyObject::None),
            Type::Ellipsis => Ok(PyObject::Ellipsis),
            Type::False => Ok(PyObject::Bool(false)),
            Type::True => Ok(PyObject::Bool(true)),
            Type::Int => {
                let obj = PyObject::Int(self.read_int());
                Ok(self.r_ref(obj, flag))
            }
            Type::Int64 => {
                let obj = PyObject::Int64(self.read_long());
                Ok(self.r_ref(obj, flag))
            }
            Type::Long => {
                let obj = PyObject::Long(self.read_pylong());
                Ok(self.r_ref(obj, flag))
            }
            Type::Float => {
                let obj = PyObject::Float(self.read_float_from_str());
                Ok(self.r_ref(obj, flag))
            }
            Type::BinaryFloat => {
                let obj = PyObject::Float(self.read_float_from_bin());
                Ok(self.r_ref(obj, flag))
            }
            Type::Complex => {
                let obj = PyObject::Complex(self.read_float_from_str(), self.read_float_from_str());
                Ok(self.r_ref(obj, flag))
            }
            Type::BinaryComplex => {
                let obj = PyObject::Complex(self.read_float_from_bin(), self.read_float_from_bin());
                Ok(self.r_ref(obj, flag))
            }
            Type::String => {
                let length = self.read_int();
                let buf = self.read_bytes(length as usize);
                let obj = PyObject::String(buf.to_vec());
                Ok(self.r_ref(obj, flag))
            }
            Type::Ascii | Type::AsciiInterned => {
                let length = self.read_int();
                let buf = self.read_bytes(length as usize);
                let obj = PyObject::AsciiString(std::str::from_utf8(buf)?.to_string());
                Ok(self.r_ref(obj, flag))
            }
            Type::ShortAscii | Type::ShortAsciiInterned => {
                let length = self.read_byte();
                let buf = self.read_bytes(length as usize);
                let obj = PyObject::AsciiString(std::str::from_utf8(buf)?.to_string());
                Ok(self.r_ref(obj, flag))
            }
            Type::Interned | Type::Unicode => {
                let length = self.read_int();
                let buf = self.read_bytes(length as usize);
                let obj = PyObject::String(buf.to_vec());
                Ok(self.r_ref(obj, flag))
            }
            Type::SmallTuple => {
                let length = self.read_byte();
                let idx = self.read_ref_reserve(flag);
                let mut retval = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    retval.push(self.read_object());
                }
                let obj = PyObject::Tuple(retval);
                Ok(self.insert_ref(obj, idx, flag))
            }
            Type::Tuple => {
                let length = self.read_int();
                let idx = self.read_ref_reserve(flag);
                let mut retval = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    retval.push(self.read_object());
                }
                let obj = PyObject::Tuple(retval);
                Ok(self.insert_ref(obj, idx, flag))
            }
            Type::List => {
                let length = self.read_int();
                let mut retval = Vec::with_capacity(length as usize);
                let origin_refs_len = self.refs.len();
                self.r_ref(PyObject::List(vec![]), flag);
                for _ in 0..length {
                    retval.push(self.read_object());
                }
                let obj = PyObject::List(retval);
                #[cfg(debug_assertions)]
                {
                    assert!(self.refs.len() > origin_refs_len);
                }
                self.refs[origin_refs_len] = obj.clone();
                Ok(obj)
            }
            Type::Dict => {
                let mut retval = Vec::new();
                let origin_refs_len = self.refs.len();
                self.r_ref(PyObject::Dict(vec![]), flag);
                loop {
                    let key = self.read_object();
                    if key == PyObject::Null {
                        break;
                    }
                    let val = self.read_object();
                    retval.push((key, val));
                }
                let obj = PyObject::Dict(retval);
                #[cfg(debug_assertions)]
                {
                    assert!(self.refs.len() > origin_refs_len);
                }
                self.refs[origin_refs_len] = obj.clone();
                Ok(obj)
            }
            Type::Set => {
                let length = self.read_int();
                let origin_refs_len = self.refs.len();
                self.r_ref(PyObject::Set(vec![]), flag);
                let mut retval = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    retval.push(self.read_object());
                }
                let obj = PyObject::Set(retval);
                #[cfg(debug_assertions)]
                {
                    assert!(self.refs.len() > origin_refs_len);
                }
                self.refs[origin_refs_len] = obj.clone();
                Ok(obj)
            }
            Type::FrozenSet => {
                let length = self.read_int();
                let mut retval = Vec::with_capacity(length as usize);
                let idx = self.read_ref_reserve(flag);
                for _ in 0..length {
                    retval.push(self.read_object());
                }
                let obj = PyObject::Set(retval);
                Ok(self.insert_ref(obj, idx, flag))
            }
            Type::Code => {
                let origin_refs_len = self.refs.len();
                self.r_ref(PyObject::None, flag);
                let retval = Code {
                    arg_count: self.read_int() as u32,
                    pos_only_arg_count: self.read_int() as u32,
                    kw_only_arg_count: self.read_int() as u32,
                    stack_size: self.read_int() as u32,
                    flags: self.read_int() as u32,
                    code: self.read_object(),
                    consts: self.read_object(),
                    names: self.read_object(),
                    locals_plus_names: self.read_object(),
                    locals_plus_kinds: self.read_object(),
                    file_name: self.read_object(),
                    name: self.read_object(),
                    qual_name: self.read_object(),
                    first_line_no: self.read_int() as u32,
                    line_table: self.read_object(),
                    exception_table: self.read_object(),
                };
                let obj = PyObject::Code(Box::new(retval));
                #[cfg(debug_assertions)]
                {
                    assert!(self.refs.len() > origin_refs_len);
                }
                self.refs[origin_refs_len] = obj.clone();
                Ok(obj)
            }
            Type::Ref => {
                let n = self.read_int() as usize;
                let obj = self.refs.get(n).ok_or("Index out of range")?.clone();
                if obj == PyObject::None {
                    Err("Cannot reference NoneType".into())
                } else {
                    Ok(obj)
                }
            }
            _ => Err(format!("Unknown type: {:?} {}", co_type, code_byte).into()),
        }
    }
}

pub fn loads(data: &[u8]) -> PyObject {
    let reader = &mut Reader::new(data);
    reader.read_object()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_loads() {
        let data = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "../../../test/pyc_test/__pycache__/demo1.cpython-311.pyc"
        ));
        println!("{:?}", &data[16..]);

        let obj = loads(&data[16..]);
        println!("{:?}", obj);
        //assert!(false);
    }
}
