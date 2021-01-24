use rustyline::error::ReadlineError;
use rustyline::Editor;

use cfg_if::cfg_if;

use sri_lang::Compile;
cfg_if! {
    if #[cfg(feature = "jit")] {
        use sri_lang::Jit as Engine;
    }
    else if #[cfg(feature = "interpreter")] {
        use sri_lang::Interpreter as Engine;
    }
    else if #[cfg(feature = "vm")]{
        use sri_lang::vm::bytecode::Interpreter as Engine;
        use sri_lang::VM;
    }
}
fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
