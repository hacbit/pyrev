//! Pyrev plugin for pyc parser

/// object
/// This file is to define the PyObject
mod object;

/// marshal
/// This file is rewritten from the marshal module in Python
/// It only supports the marshal.load function
mod marshal;

/// opcode
/// This file implements parsing the opcode(u16/u8) to the opcode name (defined in pyrev_core::opcode)
/// and the map table is according different python version
mod opcode;

/// prelude
/// export the loads function from marshal
/// example:
/// ```rust
/// use pyrev_pyc::prelude::*;
/// let code_bytes = &[227, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 243, 136, 0, 0, 0, 151, 0, 100, 0, 100, 1, 108, 0, 84, 0, 2, 0, 101, 1, 166, 0, 0, 0, 171, 0, 0, 0, 0, 0, 0, 0, 0, 0, 90, 2, 2, 0, 101, 3, 101, 2, 160, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 166, 0, 0, 0, 171, 0, 0, 0, 0, 0, 0, 0, 0, 0, 166, 1, 0, 0, 171, 1, 0, 0, 0, 0, 0, 0, 0, 0, 90, 5, 2, 0, 101, 6, 100, 2, 166, 1, 0, 0, 171, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 0, 101, 6, 101, 5, 166, 1, 0, 0, 171, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 100, 3, 83, 0, 41, 4, 233, 0, 0, 0, 0, 41, 1, 218, 1, 42, 122, 15, 98, 97, 115, 101, 54, 52, 32, 100, 101, 99, 111, 100, 101, 58, 32, 78, 41, 7, 218, 6, 98, 97, 115, 101, 54, 52, 218, 5, 105, 110, 112, 117, 116, 218, 4, 102, 108, 97, 103, 218, 9, 98, 54, 52, 100, 101, 99, 111, 100, 101, 218, 6, 101, 110, 99, 111, 100, 101, 218, 3, 100, 101, 99, 218, 5, 112, 114, 105, 110, 116, 169, 0, 243, 0, 0, 0, 0, 250, 10, 46, 47, 100, 101, 109, 111, 49, 46, 112, 121, 250, 8, 60, 109, 111, 100, 117, 108, 101, 62, 114, 14, 0, 0, 0, 1, 0, 0, 0, 115, 85, 0, 0, 0, 240, 3, 1, 1, 1, 216, 0, 20, 208, 0, 20, 208, 0, 20, 208, 0, 20, 216, 7, 12, 128, 117, 129, 119, 132, 119, 128, 4, 216, 6, 15, 128, 105, 144, 4, 151, 11, 146, 11, 145, 13, 148, 13, 209, 6, 30, 212, 6, 30, 128, 3, 216, 0, 5, 128, 5, 208, 6, 23, 209, 0, 24, 212, 0, 24, 208, 0, 24, 216, 0, 5, 128, 5, 128, 99, 129, 10, 132, 10, 128, 10, 128, 10, 128, 10, 114, 12, 0, 0, 0];
/// let code = loads(code_bytes);
/// println!("{:?}", code);
/// assert_eq!(
///     code,
///     Code(Box::new(Code {
///         arg_count: 0,
///         pos_only_arg_count: 0,
///         kw_only_arg_count: 0,
///         stack_size: 4,
///         flags: 0,
///         code: String(vec![151, 0, 100, 0, 100, 1, 108, 0, 84, 0, 2, 0, 101, 1, 166, 0, 0, 0, 171, 0, 0, 0, 0, 0, 0, 0, 0, 0, 90, 2, 2, 0, 101, 3, 101, 2, 160, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 166, 0, 0, 0, 171, 0, 0, 0, 0, 0, 0, 0, 0, 0, 166, 1, 0, 0, 171, 1, 0, 0, 0, 0, 0, 0, 0, 0, 90, 5, 2, 0, 101, 6, 100, 2, 166, 1, 0, 0, 171, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 0, 101, 6, 101, 5, 166, 1, 0, 0, 171, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 100, 3, 83, 0]),
///         consts: Tuple(vec![
///             Int(0),
///             Tuple(vec![
///                 AsciiString("*".to_owned())
///             ]),
///             AsciiString("base64 decode: ".to_owned()),
///             None
///         ]),
///         names: Tuple(vec![
///             AsciiString("base64".to_owned()),
///             AsciiString("input".to_owned()),
///             AsciiString("flag".to_owned()),
///             AsciiString("b64decode".to_owned()),
///             AsciiString("encode".to_owned()),
///             AsciiString("dec".to_owned()),
///             AsciiString("print".to_owned())
///         ]),
///         locals_plus_names: Tuple(vec![]),
///         locals_plus_kinds: String(vec![]),
///         file_name: AsciiString("./demo1.py".to_owned()),
///         name: AsciiString("<module>".to_owned()),
///         qual_name: AsciiString("<module>".to_owned()),
///         first_line_no: 1,
///         line_table: String(vec![240, 3, 1, 1, 1, 216, 0, 20, 208, 0, 20, 208, 0, 20, 208, 0, 20, 216, 7, 12, 128, 117, 129, 119, 132, 119, 128, 4, 216, 6, 15, 128, 105, 144, 4, 151, 11, 146, 11, 145, 13, 148, 13, 209, 6, 30, 212, 6, 30, 128, 3, 216, 0, 5, 128, 5, 208, 6, 23, 209, 0, 24, 212, 0, 24, 208, 0, 24, 216, 0, 5, 128, 5, 128, 99, 129, 10, 132, 10, 128, 10, 128, 10, 128, 10]),
///         exception_table: String(vec![])
///     }))
/// )
/// ```
/// export some Python Object definition in object.rs
/// you can use enum variant in PyObject no long need to use PyObject::xxx
pub mod prelude {
    use std::path::PathBuf;

    pub use crate::marshal::loads;
    pub use crate::object::{Code, PyLong, PyObject::*};
    pub use pyrev_app::prelude::*;

    pub struct PycPlugin;

    impl Plugin for PycPlugin {
        fn subcommand(&self) -> Command {
            Command::new("pyc").about("decompile pyc files").arg(
                Arg::new("file")
                    .short('f')
                    .help("specify a pyc file")
                    .action(ArgAction::Set)
                    .required(false)
                    .value_parser(value_parser!(PathBuf)),
            )
        }

        fn run(&self, args: &ArgMatches) -> Result<()> {
            let pyc_path = args
                .try_get_one::<PathBuf>("file")?
                .ok_or("File not found")?;

            info!("Decompiling {:?}", pyc_path);

            let data = std::fs::read(pyc_path)?;
            let code = loads(&data[16..]);

            println!("{:?}", code);

            Ok(())
        }
    }
}
