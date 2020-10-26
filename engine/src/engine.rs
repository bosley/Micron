/*

        TODO : 

            The execution of unary ops and opcodes needs to be copied over from the original implementation

            Certain accessors for variables aren't created (see the TODO in the perform_accessor method)

            Built in function '$drop(Vec<Expr>)' needs to be added to grammar and the engine to delete things

            Maybe merge in the $to_<type> built-ins into accessors "." because why not ? 

            String representation of dictionaries could be prettier
*/


extern crate micron_ast;

use std::{ cell::RefCell, rc::Rc };

use micron_ast::{ Statement, Expr, VariableType, DictAccessType, Accessors, MemberMethod};
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

    /// Get a record from the operational dictionary if it exists by the Variable Type (Singular v.s Nested)
    /// This will return an editable value
    fn get_record_by_var_type(&self, var_type: VariableType) -> Option<Rc<RefCell<RecordData>>> {

        match var_type {
            VariableType::Singular(var_name) => {

                return self.get_record(&var_name);
            }

            //  This will drill into the any n-dictionaries and I'm very proud of it
            //
            VariableType::Nested(var_name, accessor) => {

                // First we get the top level variable
                let mut top_level_variable = match self.get_record(&var_name) {
                    Some(existing_variable) => {
                        Box::new( existing_variable)
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

                                    top_level_variable = Box::new(new_value)
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

                                            return Some(new_value);
                                        }

                                        _ => {

                                            eprintln!("Variable for dictionary key is not a string!");
                                            return None;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            return None;
                        }
                    }
                }
                return Some(*top_level_variable);
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

                    // Assign a simple singular variable i.e   a = 3;
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

                    // Assign a more complicated 'nested' variable i.e  a['key_1']['key_2'] = "Some value"
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

    /// Execute an actual expression 
    fn execute_expression(&mut self, expression: Expr) -> Option<ExecutionError> {

        match expression {

            // Load a raw integer
            //
            Expr::Number(i) => {
                self.op_stack.push(Rc::new(RefCell::new(RecordData::Integer(i))));
                return None;
            }

            // Load a raw real
            //
            Expr::Real(f) => {
                self.op_stack.push(Rc::new(RefCell::new(RecordData::Float(f))));
                return None;
            }

            // Load a raw string
            //
            Expr::String(s) => {
                self.op_stack.push(Rc::new(RefCell::new(RecordData::String(s))));
                return None;
            }

            // Load a variable
            //
            Expr::Variable(v) => {

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

            //  Load a new dictionary
            //
            Expr::Dict(dict_entries) => {

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

            // Modify some variable(s) with a built in function
            //
            Expr::BuiltInModifierCall(modifier_function, variables) => {

                return self.process_modifier(modifier_function, variables);
            }

            //  Access
            //
            Expr::Access(access_expr, accessor, method) => {

                return self.perform_access(*access_expr, accessor, *method);
            }

            Expr::UnaryOp(op_expr, o) => {

                panic!("Unary op not yet complete!");
            }

            Expr::Op(lhs_expr, op, rhs_expr) => {

                panic!("Op not yet complete!");
            }
        }


        None
    }

    /// Process a modification 
    fn process_modifier(&mut self, modifier_function: String, variables: Vec<VariableType>) -> Option<ExecutionError> {

        println!("Modifier : {} | Variables : {:?}", modifier_function, variables);

        match modifier_function.as_str() {

            "to_string" => {
                
                for v in variables {

                    // Get the variable from memory
                    match self.get_record_by_var_type(v) {
                        Some(var) => {
                            let mut var_item = var.borrow_mut();
                            match var_item.to_string() {
                                Some(v) => { var_item.update_value(v) }
                                None    => { return Some(ExecutionError::ConversionFailure(modifier_function, "Convert item to string".to_string())) }
                            }
                        }
                        None => {
                            
                            // Otherwise its an error
                            return Some(ExecutionError::UnknownVariable)
                        }
                    };
                }
                
                self.op_stack.push(Rc::new(RefCell::new(
                    RecordData::Integer(rug::Integer::from(1))
                )));
                return None;
            }

            "to_int" => {

                for v in variables {

                    // Get the variable from memory
                    match self.get_record_by_var_type(v) {
                        Some(var) => {
                            let mut var_item = var.borrow_mut();
                            match var_item.to_int() {
                                Some(v) => { var_item.update_value(v) }
                                None    => { return Some(ExecutionError::ConversionFailure(modifier_function, "Convert item to int".to_string())) }
                            }
                        }
                        None => {
                            
                            // Otherwise its an error
                            return Some(ExecutionError::UnknownVariable)
                        }
                    };
                }
                
                self.op_stack.push(Rc::new(RefCell::new(
                    RecordData::Integer(rug::Integer::from(1))
                )));
                return None;
            }

            "to_float" => {

                for v in variables {

                    // Get the variable from memory
                    match self.get_record_by_var_type(v) {
                        Some(var) => {
                            let mut var_item = var.borrow_mut();
                            match var_item.to_float() {
                                Some(v) => { var_item.update_value(v) }
                                None    => { return Some(ExecutionError::ConversionFailure(modifier_function, "Convert item to float".to_string())) }
                            }
                        }
                        None => {
                            
                            // Otherwise its an error
                            return Some(ExecutionError::UnknownVariable)
                        }
                    };
                }
                
                self.op_stack.push(Rc::new(RefCell::new(
                    RecordData::Integer(rug::Integer::from(1))
                )));
                return None;
            }

            _ => {
                return Some(ExecutionError::UnknownBuiltInFunction(modifier_function));
            }
        }
    }

    /// Perform actions on an accessor
    fn perform_access(&mut self, item_expr: Expr, accessor: Accessors, method: MemberMethod) -> Option<ExecutionError> {

        // Load the variable to access
        if let Some(err) = self.execute_expression(item_expr) {
            return Some(err);
        }

        // Get the item
        let accessed_item = match self.op_stack.pop() {
            None => {
                return Some(ExecutionError::StackError);
            }

            Some(val) => { 
                val
             }
        };

        /*
        
                TODO : .set_precision()     - Modify the actual data item
                       .with_precision()    - Make a copy with given precision
                       .at()                - Copy single string char
        
        */

        match accessor {

            Accessors::Dot => {

                match method.method.as_str() {

                    "as_string" => {
                        let mut new_item = accessed_item.borrow().get_value();
                        match new_item.to_string() {
                            Some(v) => { self.op_stack.push(Rc::new(RefCell::new(v.clone()))) }
                            None    => { return Some(ExecutionError::ConversionFailure(method.method, "Represent item as string".to_string())) }
                        }
                        None
                    }

                    "as_int" => {
                        let mut new_item = accessed_item.borrow().get_value();
                        match new_item.to_int() {
                            Some(v) => { self.op_stack.push(Rc::new(RefCell::new(v.clone()))) }
                            None    => { return Some(ExecutionError::ConversionFailure(method.method, "Represent item as int".to_string())) }
                        }
                        None
                    }

                    "as_float" => {
                        let mut new_item = accessed_item.borrow().get_value();
                        match new_item.to_float() {
                            Some(v) => { self.op_stack.push(Rc::new(RefCell::new(v.clone()))) }
                            None    => { return Some(ExecutionError::ConversionFailure(method.method, "Represent item as float".to_string())) }
                        }
                        None
                    }
                    _ => {

                        return Some(ExecutionError::UnknownVariableMethod(".", method.method));
                    }
                }

            }
        }
    }

}