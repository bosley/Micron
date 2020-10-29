
/*

    Read in a file line by line. Remove all comments and conjoin into single string, or series of parsable strings. 
    Once read in, send to parser. From the parser the statements can be handed to caller to do with as they please
*/

use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Debug)]
pub enum ImportError {
    UnableToOpenFile,
    LineReadError,
    ParseError
}

#[derive(Debug)]
pub struct ImportResult {
    pub statements: Vec<Box<micron_ast::Statement>>
}

pub fn import(file: &str) -> Result<ImportResult, ImportError> {

    /*
        I really don't like how this importer is working, but I really don't care at the moment. 
        All that needs to happen is that this bad boy takes in a file and generates an ImportResult
        That result needs to hold onto ast statements. 

        In the future the importer should also detect and deserialize things if its a serialized file 
    
        We could also stream line this to detect specific points when we know we can send off to parse, instead 
        of having everything put into one big conjoined string. For big programs that would be awful!
    */

    return simple_importer(file);
}

fn simple_importer(file: &str) -> Result<ImportResult, ImportError> {

    let file_in = match File::open(file) {
        Ok(o) => o,
        Err(_) => return Err(ImportError::UnableToOpenFile)
    };

    // Reader
    let reader = BufReader::new(file_in);

    // Regex things for detecting things
    let comment_line = Regex::new(r"^\-{2}[\W+\w+]*").unwrap();

    let mut conjoined_input : String = String::new();

    // Iter over lines and build things
    for (_index, line) in reader.lines().enumerate() {

        let mut line = match line {
            Ok(l) => l,
            Err(_) => return Err(ImportError::LineReadError)
        };

        // Get rid of all that ws
        line = line.trim_start().to_string().trim_end().to_string();

        // If there isn't anything on the line, ignore it
        if line.len() == 0 {
            continue;
        }

        // If there is a comment on the line, then we want to ignore it
        if comment_line.is_match(line.as_str()) {
            continue;
        }

        // Conjoin the new line into the others with a space sep 
        // This is a bad design decision and should change once things get more solid with the language
        // This SERIOUSLY needs to be revisited 
        conjoined_input = conjoined_input + " " + line.as_str();

        //println!("{}. {}", index + 1, line);
    }

    // println!("Result : {}", conjoined_input);

    // Now that we have the giant string of input we feed it to the program parser to generate statements.
    // These are the statements that we'll hand back to the caller. When we deserialize things we won't need
    // to do this part, and instead we will just load them up and send them off from the file
    match micron_parser::micron::ProgramParser::new().parse(&conjoined_input) {

        Ok(statements)  => { 
            
            return Ok(ImportResult{
                statements: statements
            });
        }
        
        Err(e) => { 
            panic!("Error >>> {}", e);
        }
    };
}