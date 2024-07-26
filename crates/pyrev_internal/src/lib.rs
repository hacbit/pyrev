//! # Pyrev internal
//!
//! This is the internal crate for the pyrev project. It contains the core logic for the project.

/// The prelude module contains all the public exports from the other modules in the crate.
pub mod prelude {
    pub use pyrev_app::prelude::*;
    pub use pyrev_ast::*;
    pub use pyrev_ast_build::*;
    pub use pyrev_ast_visit::*;
    pub use pyrev_core::*;
    pub use pyrev_decompiler::*;
    pub use pyrev_log::*;
    pub use pyrev_marshal::*;
    pub use pyrev_object::*;
    pub use pyrev_parser::*;
    pub use pyrev_plugin::*;
    pub use pyrev_query::*;
}
