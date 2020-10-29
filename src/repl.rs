
extern crate micron_engine;

use micron_engine::Engine;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
   repl();
}

fn repl_banner() {
    println!("           ██████   ██████    ");
    println!("          ░░██████ ██████     ");
    println!("█████ ████ ░███░█████░███     ");
    println!("░░███ ░███ ░███░░███ ░███     ");
    println!("░███ ░███  ░███ ░░░  ░███     Micron Language REPL   ");
    println!("░███ ░███  ░███      ░███     --------------------   ");
    println!("░░████████ █████     █████    Author : Josh A. Bosley");
    println!("░░░░░░░░   ░░░░░     ░░░░░    License: MIT           ");
    println!("\n\n");
}

fn repl() {

    repl_banner();

    let mut engine  = Engine::new();

    let mut rl = Editor::<()>::new();
    if rl.load_history("repl-history.txt").is_err() {
        println!("No previous repl history.");
    }

    loop {

        match rl.readline(">> ") {

            Ok(line) => {
                rl.add_history_entry(line.as_str());

                match micron_parser::micron::ProgramParser::new().parse(&line) {

                    Ok(statements)  => { 
                        
                        for x in statements {
            
                            if let Some(e) = engine.execute_statement(*x) {

                                println!("Error: {}", e);
                            }
                        } 
                    }
                    Err(e) => { 
                        println!("Error >>> {}", e);
                        continue;
                    }
                };

            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                //println!("CTRL-D");
                break
            },
            Err(_err) => {
                //println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("repl-history.txt").unwrap();
}
