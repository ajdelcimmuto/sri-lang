pub mod compiler;
pub mod parser;
pub use crate::compiler::interpreter::Interpreter;

pub trait Compile {
    fn from_source(src: &str) {
        parser::parse(src);
    }
}
