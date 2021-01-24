use sri_lang::Compiler;

pub struct Interpreter;

impl Compiler for Interpreter {
    type Output = Result<i32>;
}
