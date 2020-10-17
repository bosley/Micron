

mod env;
use env::Environment;

mod eval;
use eval::Eval;

mod ast;

use rustyline::error::ReadlineError;
use rustyline::Editor;

#[macro_use] 
extern crate lalrpop_util;

lalrpop_mod!(pub micron); // synthesized by LALRPOP


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

    let mut env  = Environment::new();
    let mut eval = Eval::new(&mut env);

    let mut rl = Editor::<()>::new();
    if rl.load_history("repl-history.txt").is_err() {
        println!("No previous repl history.");
    }

    loop {

        match rl.readline(">> ") {

            Ok(line) => {
                rl.add_history_entry(line.as_str());

                match micron::ProgramParser::new().parse(&line) {

                    Ok(statements)  => { 
                        
                        for x in statements {
            
                            eval.evaluate_statement(*x);
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
