use sri_lang::Compile;

mod compiler;
use compiler::interpreter::Interpreter as Engine;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("No file argument found");
        std::process::exit(-1);
    }

    let source: &str = &std::fs::read_to_string(&args[1]).unwrap();
    Engine::from_source(source);
}
