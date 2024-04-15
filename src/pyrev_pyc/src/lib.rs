//! Pyrev plugin for pyc parser
//! 


/// object
/// This file is to define the PyObject
mod object;

/// marshal
/// This file is rewritten from the marshal module in Python
/// It only supports the marshal.load function
mod marshal;


pub mod prelude {
    pub use crate::marshal::loads;
}