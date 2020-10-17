use std::collections::HashMap;

mod object;
pub use object::Object;

/// Scopes
struct Scope {

    variables: HashMap<String, Object>
}

impl Scope {

    fn new() -> Self {
        Self {
            variables: HashMap::new()
        }
    }
}

/// The micron environment
pub struct Environment {

    scopes: HashMap<String, Scope>,
    current_scope: String
}

impl Environment {

    /// Create a new environment
    pub fn new() -> Self {

        // Create the env
        let mut e = Environment {
            scopes: HashMap::new(),
            current_scope: String::from("global")
        };

        e.scopes.insert(String::from("global"), Scope::new());

        // Return env
        e
    }

    /// Set a variable
    pub fn set_variable(&mut self, key:&String, value: Object) {

        match self.scopes.get_mut(&self.current_scope) {
            Some(r_scope) => {
                
                r_scope.variables.insert(key.clone(), value.clone());
            }

            None => {
                panic!("Unable to find scope : {}", self.current_scope);
            }
        }
    }

    /// Get a variable
    pub fn get_variable(&mut self, key: &String) -> Option<Object> {

        match self.scopes.get(&self.current_scope) {
            Some(r_scope) => {
                
                match r_scope.variables.get(key) {

                    Some(var) => { return Some(var.clone()); }
                    None      => { return None; }
                }
            }

            None => {
                panic!("Unable to find scope : {}", self.current_scope);
            }
        }
    }
}