use std::collections::HashMap;

mod object;
pub use object::Object;

use crate::ast;

struct Scope {

    variables: HashMap<String, Object>
}

pub struct Environment {

    scopes: Vec<Scope>
}

impl Environment {

    /// Create a new environment
    pub fn new() -> Self {

        // Create the env
        let mut e = Environment {
            scopes: Vec::new()
        };

        e.scopes.push(Scope{
            variables: HashMap::new()
        });

        // Return env
        e
    }

    pub fn set_variable(&mut self, key: String, value: Object) {

    }

    pub fn get_variable(&mut self, key: &String) -> Option<Object> {

   

        None
    }
}