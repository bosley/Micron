use std::collections::HashMap;

use crate::object::{ Object /*, get_object_from_dict_item */ };
use crate::error::EnvError;
use crate::types::{ MString };

/// Object Scope
struct Scope {

    variables: HashMap<String, Object>
}

impl Scope {

    // Create a new scope
    fn new() -> Self {
        Scope {
            variables: HashMap::new()
        }
    }
}

/// The Micron Environment
pub struct MicronEnv {
    
    scopes: HashMap<String, Scope>,
    current_scope:  String
}

impl MicronEnv {

    /// Create a new Micron Environment
    pub fn new() -> Self {

        let mut m_env = MicronEnv {
            scopes: HashMap::new(),
            current_scope: String::from("global")
        };

        m_env.scopes.insert("global".to_string(), Scope::new());

        m_env
    }

    /// Get a variable from the environment
    /// If no scope is given the 'current scope' is used
    pub fn get_variable(&self, key: MString, scope: Option<String>) -> Result<Object, EnvError> {

        // Check if they want to search a particular scope
        let scope = match scope {
            Some(s) => { s }
            None    => { self.current_scope.clone() }
        };

        match self.scopes.get(&scope) {
            Some(r_scope) => {
                
                match r_scope.variables.get(&key.get_value()) {
                    Some(var) => { return Ok(var.clone()); }
                    None      => { return Err(EnvError::UnknownVariable(key.get_value())); }
                }
            }

            None => {
                return Err(EnvError::UnknownScope("Unknown scope: ".to_owned() + scope.as_str()))
            }
        }
    }

    /// Set a variable
    /// If no scope is given the 'current scope' is used
    pub fn set_variable(&mut self, key:&String, value: Object, scope: Option<String>) -> Result<(), EnvError> {

        let scope = match scope {
            Some(s) => { s }
            None    => { self.current_scope.clone() }
        };

        match self.scopes.get_mut(&scope) {
            Some(r_scope) => {
                
                r_scope.variables.insert(key.clone(), value.clone());
                return Ok(());
            }

            None => {
                return Err(EnvError::UnknownScope("Unknown scope: ".to_owned() + scope.as_str()))
            }
        }
    }
}