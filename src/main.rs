use cfg_if::cfg_if;

use sri_lang::Compile;

use sri_lang::Interpreter as Engine;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("No file argument found");
        std::process::exit(-1);
    }
    println!(
        "{:?}",
        Engine::from_source(&std::fs::read_to_string(&args[1]).unwrap()).unwrap()
    );
}
