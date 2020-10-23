
extern crate micron_ast;

use std::{ cell::RefCell, rc::Rc };

use micron_ast::{ Statement, Expr, VariableType, DictAccessType};
use crate::types::{ Dictionary, RecordData };
use crate::error::ExecutionError;

/// The Micron Engine 
#[derive(Debug, Clone)]
pub struct Engine {

    /// Stored data
    scopes: Vec<Dictionary>,
    op_stack: Vec<Rc<RefCell<RecordData>>>
}

impl Engine {

    /// Create a new engine
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
            op_stack: Vec::new()
        }
    }

    /// Add a new scope to the scope list
    fn new_scope(&mut self) {
        self.scopes.push(Dictionary::new());
    }

    /// Remove a scope from the scope list. 
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Get a record - Traverses scopes in reverse in an attempt
    /// to find the requested record. The first one found will be returned
    fn get_record(&self, key: &String) -> Option<Rc<RefCell<RecordData>>> {
        for scope in self.scopes.iter().rev() {
            match scope.get(key) {
                Some(record) => { return Some(record); }
                None         => { return None; }
            }
        }
        None
    }

    /// Get the current scope
    fn current_scope (&mut self) -> &mut Dictionary {

        // If there is no scope for some reason
        if self.scopes.len() == 0 {

            // add  a new scope
            self.new_scope();
        }

        let n = self.scopes.len();

        // Return a mut reference to the top scope
        &mut self.scopes[n-1]
    }

    /// Set a record
    fn set_record(&mut self, key: &String, record: RecordData) {
        
        self.current_scope().set(key, record);
    }

    fn get_record_by_var_type(&self, var_type: VariableType) -> Option<Rc<RefCell<RecordData>>> {

        /*
        
                This isn't working the way I meant it to. 

                Its updating the referenced item to the inner and changing it rather than changing 
                what its referencing. This is not what we need.

                We need to change what item its referencing, not change the referenced item its self. 

                We could solve this by writing a helper method and making this recursive. Oof.
        
        
        */

        eprintln!("THIS IS NOT WORKING CORRECTLY");

        match var_type {
            VariableType::Singular(var_name) => {

                return self.get_record(&var_name);
            }

            VariableType::Nested(var_name, accessor) => {

                // First we get the top level variable
                let top_level_variable = match self.get_record(&var_name) {
                    Some(existing_variable) => {
                        existing_variable
                    }
                    None => { return None; }
                };

                // For every item in the accessor list we drill into the dictionaries
                for item in accessor.iter() {

                    let top_level_val = top_level_variable.borrow().get_value();

                    // Ensure the current item is a dictionary
                    match top_level_val {
                        RecordData::Dict(dictionary) => {

                            // See what the accessor type is
                            match item {

                                // If its a raw string then we just have to get its item
                                DictAccessType::RawValue(string_key) => {
        
                                    // Set the top level variable to its inner item
                                    let new_value = match dictionary.get(&string_key) {
                                        Some(val) => { val }
                                        None => { 
                                            eprintln!("Unable to find record for key '{}'", string_key);
                                            return None 
                                        }
                                    };

                                    top_level_variable.borrow_mut().update_value(new_value.borrow().clone());
                                }
            
                                // If its a variable we have to load the variable and ensure its a string first
                                // once thats done we can update the item 
                                DictAccessType::Variable(var_key) => {

                                    let suspected_string_var = match self.get_record(&var_key) {
                                        Some(val) => { val }
                                        None => { 
                                            eprintln!("Could not find string key '{}'", var_key);
                                            return None 
                                        }
                                    };

                                    let suspect_value = suspected_string_var.borrow().get_value();

                                    match suspect_value {
                                        RecordData::String(string_key) => {

                                            // Set the top level variable to its inner item
                                            let new_value = match dictionary.get(&string_key) {
                                                Some(val) => { val }
                                                None => { 
                                                    eprintln!("Unable to find record for key '{}'", string_key);
                                                    return None 
                                                }
                                            };

                                            top_level_variable.borrow_mut().update_value(new_value.borrow().clone());
                                        }

                                        _ => {

                                            eprintln!("Variable for dictionary key is not a string!");
                                            return None;
                                        }
                                    }
                                    return None;
                                }
                            }
                        }
                        _ => {
                            return None;
                        }
                    }
                }
                return Some(top_level_variable);
            }
        }
    }

    /// Execute an AST statement
    pub fn execute_statement(&mut self, statement: Statement) -> Option<ExecutionError> {

        /*

            Match the statement and do what it asks
        
        */
        match statement {

            Statement::Assignment(var_type, expr) => {

                // Clear operational stack just in case
                self.op_stack.clear();

                match var_type.clone() {
                    VariableType::Singular(var_name) => {

                        match self.execute_expression(*expr) {

                            Some(e) => { return Some(e); }

                            None => {

                                let value = match self.op_stack.pop() {
                                    None => {
                                        return Some(ExecutionError::StackError);
                                    }

                                    Some(val) => { val }
                                };

                                self.set_record(&var_name, value.borrow().clone());
                            }
                        }
                    }

                    VariableType::Nested(_, _) => {

                        match self.execute_expression(*expr) {

                            Some(e) => { return Some(e); }

                            None => {

                                match self.op_stack.pop() {
                                    None => {
                                        return Some(ExecutionError::StackError);
                                    }

                                    Some(val) => { 

                                        // Nested variables are expected to exist already
                                        match self.get_record_by_var_type(var_type) {

                                            Some(variable) => {
                                                let mut lhs = variable.borrow_mut();
                                                lhs.update_value(val.borrow().clone());
                                            }
                                            None => {
                                                return Some(ExecutionError::UnknownVariable);
                                            }
                                        }
                                     }
                                }
                            }
                        }
        

                        panic!("Nested variable assignment is not yet supported");
                    }
                }


                }

            Statement::BareExpression(expr) => {

                // Execute the expression
                match self.execute_expression(*expr) {

                    Some(e) => { return Some(e); }

                    None => {

                        // Get the resulting expression
                        let value = match self.op_stack.pop() {
                            None => {
                                return Some(ExecutionError::StackError);
                            }

                            Some(val) => { val.borrow().get_value() }
                        };

                        // Print the value for now
                        println!("{:?}", value);
                    }
                }
            }
        }

        None
    }

    fn execute_expression(&mut self, expression: Expr) -> Option<ExecutionError> {

        match expression {


            Expr::Number(i) => {
                println!("Pushing number");

                self.op_stack.push(Rc::new(RefCell::new(RecordData::Integer(i))));
                return None;
            }

            Expr::Real(f) => {
                println!("Pushing real");
                
                self.op_stack.push(Rc::new(RefCell::new(RecordData::Float(f))));
                return None;
            }

            Expr::String(s) => {
                println!("Pushing string");
                
                self.op_stack.push(Rc::new(RefCell::new(RecordData::String(s))));
                return None;
            }

            Expr::Variable(v) => {
                println!("Pushing variable");
                
                // Get the variable from memory
                match self.get_record_by_var_type(v) {
                    Some(var) => {

                        // If it exists stack it
                        self.op_stack.push(var);
                    }
                    None => {
                        
                        // Otherwise its an error
                        return Some(ExecutionError::UnknownVariable)
                    }
                };

                // Get on out!
                return None;
            }

            Expr::UnaryOp(op_expr, o) => {

                panic!("Unary op not yet complete!");
            }

            Expr::Op(lhs_expr, op, rhs_expr) => {

                panic!("Op not yet complete!");
            }

            Expr::Modifier(var, modifier) => {

                panic!("Modifier not yet complete!");
            }

            //  Create a new dictionary
            //
            Expr::Dict(dict_entries) => {

                println!("In dict entry");

                let mut new_dict = Dictionary::new();

                for entry in dict_entries {
                    match self.execute_expression(*entry.value) {
                        Some(e) => return Some(e),
                        None => {
                            match self.op_stack.pop() {
                                None => {
                                    eprintln!("Unable to get variable from stack to set item");
                                    return Some(ExecutionError::StackError);
                                }
                                Some(val) => { 
                                    new_dict.set(&entry.key, val.borrow().get_value());
                                }
                            }
                        }
                    }
                }

                self.op_stack.push(Rc::new(RefCell::new(RecordData::Dict(new_dict))));
                return None;
            }

            //  Accessor
            //
            Expr::Access(access_expr, accessor, method) => {

                panic!("Access not yet complete!");
            }
        }


        None
    }

}