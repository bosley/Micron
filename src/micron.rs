extern crate micron_file_import;
extern crate micron_engine;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Micron")
                          .version("0.1.0")
                          .author("Josh A. Bosley <bosley117@gmail.com>")
                          .about("Executes Micron programs")
                          .arg(Arg::with_name("file")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .get_matches();


    // Using the importer we try to bring in the file and have the parser make something
    // the engine can jive with
    let import_result = match micron_file_import::import(matches.value_of("file").unwrap()) {
        Ok(result) => { result },
        Err(e)     => {
            panic!("File Error: {:?}", e);
        }
    };

    //println!("Got: {:?}", import_result);

    execute(import_result);
}

// Take the statements read in from the importer and execute them
fn execute(imported_statements: micron_file_import::ImportResult) {

    let mut engine  = micron_engine::Engine::new();

    for statement in imported_statements.statements {

        if let Some(e) = engine.execute_statement(*statement) {

            panic!("Execution Error : {}", e);
        }
    }

}