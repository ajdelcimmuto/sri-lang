pub mod ast;
pub mod compiler;
pub mod parser;

pub use crate::compiler::interpreter::Interpreter;
use anyhow;
pub type Result<T> = anyhow::Result<T>;

pub trait Compile {
    fn from_source(src: &str) {
        let result = parser::parse(src);
        println!("{:?}", result);
    }
}
