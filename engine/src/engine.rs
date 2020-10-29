/*

        TODO : 
            Certain accessors for variables aren't created (see the TODO in the perform_accessor method)

            String representation of dictionaries could be prettier
*/


use std::convert::TryFrom;
use rug::{Integer, Float, Assign, ops::Pow};
use std::{ cell::RefCell, rc::Rc };

extern crate micron_ast;
use micron_ast::{ Statement, Expr, VariableType, DictAccessType, Accessors, MemberMethod, UnaryOpcode, Opcode, RADIX, FLOAT_PRECISION};

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

    /// Attempt to remove a variable
    fn rm_record(&mut self, key: &String) -> Option<ExecutionError> {
        match self.current_scope().remove(key) {
            true => return None,
            false => return Some(ExecutionError::UnknownVariable)
        }
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

                                            top_level_variable = Box::new(new_value)
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

            // Unary operation
            //
            Expr::UnaryOp(op_expr, op) => {

                return self.perform_unary(*op_expr, op);
            }

            // Operation
            //
            Expr::Op(lhs_expr, op, rhs_expr) => {

                return self.perform_opcode(*lhs_expr, *rhs_expr, op);
            }
        }
    }

    /// Process a modification 
    fn process_modifier(&mut self, modifier_function: String, variable: String) -> Option<ExecutionError> {

        match modifier_function.as_str() {

            "drop" => {

                match self.rm_record(&variable) {
                    Some(e) => return Some(e),
                    None    => {
                        self.op_stack.push(Rc::new(RefCell::new(
                            RecordData::Integer(Integer::from(1))
                        )));
                        None
                    }
                }
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

                    "to_string" => {
                        let mut new_item = accessed_item.borrow().get_value();
                        match new_item.to_string() {
                            Some(v) => { 
                                accessed_item.borrow_mut().update_value(v.clone());
                                self.op_stack.push(Rc::new(RefCell::new(RecordData::Integer(Integer::from(1)))));
                            
                            }
                            None    => { 
                                self.op_stack.push(Rc::new(RefCell::new(RecordData::Integer(Integer::from(0)))));
                                return Some(ExecutionError::ConversionFailure(method.method, "Convert item to string".to_string())) 
                            }
                        }
                        None
                    }

                    "to_int" => {
                        let mut new_item = accessed_item.borrow().get_value();
                        match new_item.to_int() {
                            Some(v) => { 
                                accessed_item.borrow_mut().update_value(v.clone());
                                self.op_stack.push(Rc::new(RefCell::new(RecordData::Integer(Integer::from(1))))) 
                            
                            }
                            None    => { 
                                self.op_stack.push(Rc::new(RefCell::new(RecordData::Integer(Integer::from(0)))));
                                return Some(ExecutionError::ConversionFailure(method.method, "Convert item to int".to_string()))
                             }
                        }
                        None
                    }

                    "to_float" => {
                        let mut new_item = accessed_item.borrow().get_value();
                        match new_item.to_float() {
                            Some(v) => { 
                                accessed_item.borrow_mut().update_value(v.clone());
                                self.op_stack.push(Rc::new(RefCell::new(RecordData::Integer(Integer::from(1))))) 
                            
                            }
                            None    => { 
                                self.op_stack.push(Rc::new(RefCell::new(RecordData::Integer(Integer::from(0)))));
                                return Some(ExecutionError::ConversionFailure(method.method, "Convert item to float".to_string())) 
                            }
                        }
                        None
                    }

                    "at" => {

                        if method.params.len() != 1 { 
                            return Some(ExecutionError::InvalidParameters)
                        }

                        let exp = *method.params[0].clone();

                        // Evaluate precision
                        if let Some(e) = self.execute_expression(exp) {
                            return Some(e);
                        }

                        // Get the lhs operand
                        let location = match self.op_stack.pop() {
                            Some(n) => { n }
                            None => { return Some(ExecutionError::StackError); }
                        };

                        let location = location.borrow().get_value();

                        match location {

                            RecordData::Integer(i) => {

                                match accessed_item.borrow().get_value() {
                                    RecordData::String(s) => {

                                        if i > s.len() {
                                            return Some(ExecutionError::IndexError);
                                        }

                                        let s_idx = String::from(s.bytes().nth(i.to_i64().unwrap() as usize).unwrap() as char);

                                        self.op_stack.push(Rc::new(RefCell::new(RecordData::String(s_idx))));

                                        None
                                    }

                                    _ => {
                                        return Some(ExecutionError::InvalidOperation("Non-string type does not contain method '.at(N)' "));
                                    }
                                }
                            }
                            _ => {
                                return Some(ExecutionError::InvalidOperation("Parameter expected integer"));
                            }
                        }
                    }

                    _ => {

                        return Some(ExecutionError::UnknownVariableMethod(".", method.method));
                    }
                }
            }
        }
    }

    /// Perform a unary operation
    fn perform_unary(&mut self, expression: Expr, op: UnaryOpcode) -> Option<ExecutionError> {

        // Load the variable to access
        if let Some(err) = self.execute_expression(expression) {
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

        let item_value = accessed_item.borrow().get_value();

        match item_value.clone() {

            RecordData::Integer(v) => {
                return self.perform_unary_integer(v, op);
            }
            
            RecordData::Float(_) => {

                match accessed_item.borrow().get_value().to_int() {
                    Some(v) => {
                        match v {
                            RecordData::Integer(i) => {
                                return self.perform_unary_integer(i, op);
                            },
                            _ => {
                                panic!("RecordData rust function to_int came back okay but somehow didn't return int type");
                            }
                        }
                    }
                    None    => return Some(ExecutionError::ConversionFailure(
                        "Converting float to integer".to_string(),
                        "Required for unary operation".to_string()
                    ))
                };
            }

            RecordData::String(_) => {
                return Some(ExecutionError::InvalidOperation("Attempted unary operation on string type"));
            }


            RecordData::Dict(_) => {
                return Some(ExecutionError::InvalidOperation("Attempted unary operation on dictionary type"));
            }
        }
    }

    /// Perform a unary operation
    fn perform_unary_integer(&mut self, item: Integer, op: UnaryOpcode) -> Option<ExecutionError> {

        match op {
            UnaryOpcode::BwNot => {
                self.op_stack.push(Rc::new(RefCell::new(RecordData::Integer( !item ))));
            }

            UnaryOpcode::Negate => {

                if item > 0 {
                    self.op_stack.push(Rc::new(RefCell::new(RecordData::Integer( Integer::from( 0 ) ))));
                } else {
                    self.op_stack.push(Rc::new(RefCell::new(RecordData::Integer( Integer::from( 1 ) ))));
                }
            }
        }
        None
    }

    /// Perform an opcode
    pub fn perform_opcode(&mut self, lhs: Expr, rhs: Expr, op: Opcode) -> Option<ExecutionError> {

        // Load the variable to access
        if let Some(err) = self.execute_expression(lhs) {
            return Some(err);
        }

        // Get the item
        let lhs_item = match self.op_stack.pop() {
            None => {
                return Some(ExecutionError::StackError);
            }
            Some(val) => { 
                val.borrow().get_value()
             }
        };

        // Load the variable to access
        if let Some(err) = self.execute_expression(rhs) {
            return Some(err);
        }

        // Get the item
        let rhs_item = match self.op_stack.pop() {
            None => {
                return Some(ExecutionError::StackError);
            }
            Some(val) => { 
                val.borrow().get_value()
             }
        };

        // Figure out what is going on
        /*
            This block of code determines elevation of items in an expression to other types

            By figuring out what type the lhs and rhs of the expression we promote what needs
            promotion and then call on the actual op executors for the given type
        */
        match lhs_item {

            RecordData::Integer(v_lhs) => {
                match rhs_item {
                    RecordData::Integer(v_rhs) => {
                        
                        let result = self.execute_op_integer(v_lhs, v_rhs, op);
                        self.op_stack.push(Rc::new(RefCell::new(
                            RecordData::Integer( result )
                        )));
                        None
                    }
                    RecordData::Float(v_rhs) => {
                        let mut vf_lhs = Float::new(FLOAT_PRECISION);
                        vf_lhs.assign(v_lhs);

                        let result = self.execute_op_float(vf_lhs, v_rhs, op);
                        self.op_stack.push(Rc::new(RefCell::new(
                            RecordData::Float( result )
                        )));
                        None
                    }
                    RecordData::String(v_rhs) => {
                        let v_lhs = String::from(v_lhs.to_string_radix(RADIX));
                        return self.execute_op_string(v_lhs, v_rhs, op);
                    }
                    RecordData::Dict(_) => {
                        return Some(ExecutionError::InvalidOperation("No valid operation for type Dictionary"));
                    }
                }
            }

            RecordData::Float(v_lhs) => {
                match rhs_item {
                    RecordData::Integer(v_rhs) => {
                        let mut vf_rhs = Float::new(FLOAT_PRECISION);
                        vf_rhs.assign(v_rhs);
                        let result = self.execute_op_float(v_lhs, vf_rhs, op);
                        self.op_stack.push(Rc::new(RefCell::new(
                            RecordData::Float( result )
                        )));
                        None
                    }
                    RecordData::Float(v_rhs) => {
                        let result = self.execute_op_float(v_lhs, v_rhs, op);
                        self.op_stack.push(Rc::new(RefCell::new(
                            RecordData::Float( result )
                        )));
                        None
                    }
                    RecordData::String(v_rhs) => {
                        let vs_lhs = String::from(v_lhs.to_string_radix(RADIX, Some(v_lhs.prec() as usize)));
                        return self.execute_op_string(vs_lhs, v_rhs, op);
                    }
                    RecordData::Dict(_) => {
                        return Some(ExecutionError::InvalidOperation("No valid operation for type Dictionary"));
                    }
                }
            }

            RecordData::String(v_lhs) => {
                match rhs_item {
                    RecordData::Integer(v_rhs) => {
                        let v_rhs = String::from(v_rhs.to_string_radix(RADIX));
                        return self.execute_op_string(v_lhs, v_rhs, op);
                    }
                    RecordData::Float(v_rhs) => {
                        let vs_rhs = String::from(v_rhs.to_string_radix(RADIX, Some(v_rhs.prec() as usize)));
                        return self.execute_op_string(v_lhs, vs_rhs, op);
                    }
                    RecordData::String(v_rhs) => {
                        return self.execute_op_string(v_lhs, v_rhs, op);
                    }
                    RecordData::Dict(_) => {
                        return Some(ExecutionError::InvalidOperation("No valid operation for type Dictionary"));
                    }
                }
            }

            RecordData::Dict(_) => {
                return Some(ExecutionError::InvalidOperation("No valid operation for type Dictionary"));
            }

        }
    }

    /// Execute a string operation
    fn execute_op_string(&mut self, lhs: String, rhs: String, op: Opcode) -> Option<ExecutionError> {
        
        match op {
            Opcode::Add => {
                self.op_stack.push(Rc::new(RefCell::new(RecordData::String( lhs + rhs.as_str() ))));
            }

            _ => {
                return Some(ExecutionError::InvalidOperation("Invalid operation for type String"));
            }
        }

        None
    }

    /// Execute an integer operation
    fn execute_op_integer(&mut self, lhs: Integer, rhs: Integer, op: Opcode) -> Integer {

        match op {
            Opcode::Mul => {
                //println!("mul");
                return lhs * rhs;
            }
            Opcode::Div => {
                //println!("div");
                return lhs / rhs;
            }
            Opcode::Add => {
                //println!("add");
                return lhs + rhs;
            }
            Opcode::Sub => {
                //println!("sub");
                return lhs - rhs;
            }
            Opcode::Lte => {
                return Integer::from(lhs <= rhs);
            }

            Opcode::Gte => {
                return Integer::from(lhs >= rhs);
            }

            Opcode::Lt => {
                return Integer::from(lhs < rhs);
            }

            Opcode::Gt => {
                return Integer::from(lhs > rhs);
            }

            Opcode::Equal => {
                return Integer::from(lhs == rhs);
            }

            Opcode::Ne => {
                return Integer::from(lhs != rhs);
            }

            Opcode::Pow => {

                // Rust pow for i64 requires a u32 so we attempt to convert it to a u32
                // if it fails a PANIC!
                let rhs_converted = match u32::try_from(rhs) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u32 for \"pow\": {}", e);
                    }
                };

                return Integer::from(lhs.pow(rhs_converted));
        
            }

            Opcode::Mod => {
                return lhs % rhs;
            }

            Opcode::Lsh => {

                let rhs_converted = match u64::try_from(rhs) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"lsh\": {}", e);
                    }
                };

                let lhs_converted = match u64::try_from(lhs) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"lsh\": {}", e);
                    }
                };

                return Integer::from(lhs_converted << rhs_converted);
            }

            Opcode::Rsh => {

                let rhs_converted = match u64::try_from(rhs) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"rhs\": {}", e);
                    }
                };

                let lhs_converted = match u64::try_from(lhs) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"rhs\": {}", e);
                    }
                };
                
                return Integer::from(lhs_converted >> rhs_converted);
            }

            Opcode::BwXor => {
                return lhs ^ rhs
            }

            Opcode::BwOr => {
                return lhs | rhs;
            }

            Opcode::BwAnd => {
                return lhs & rhs;
            }

            Opcode::Or => {

                if lhs > 0 || rhs > 0{
                    return Integer::from(1);
                }
                return Integer::from(0);
            }

            Opcode::And => {
                
                if lhs > 0 && rhs > 0{
                    return Integer::from(1);
                }
                return Integer::from(0);
            }
        }
    }

    /// Execute a float operation
    fn execute_op_float(&mut self, lhs: Float, rhs: Float, op: Opcode) -> Float  {
        
        match op {
            Opcode::Mul => {
                //println!("mul");
                return lhs * rhs;
            }
            Opcode::Div => {
                //println!("div");
                return lhs / rhs;
            }
            Opcode::Add => {
                //println!("add");
                return lhs + rhs;
            }
            Opcode::Sub => {
                //println!("sub");
                return lhs - rhs;
            }
            Opcode::Lte => {
                let mut val = Float::new(FLOAT_PRECISION);
                val.assign(Integer::from(lhs <= rhs));
                return val;
            }

            Opcode::Gte => {
                let mut val = Float::new(FLOAT_PRECISION);
                val.assign(Integer::from(lhs >= rhs));
                return val;
            }

            Opcode::Lt => {
                let mut val = Float::new(FLOAT_PRECISION);
                val.assign(Integer::from(lhs < rhs));
                return val;
            }

            Opcode::Gt => {
                let mut val = Float::new(FLOAT_PRECISION);
                val.assign(Integer::from(lhs > rhs));
                return val;
            }

            Opcode::Equal => {
                let mut val = Float::new(FLOAT_PRECISION);
                val.assign(Integer::from(lhs == rhs));
                return val;
            }

            Opcode::Ne => {
                let mut val = Float::new(FLOAT_PRECISION);
                val.assign(Integer::from(lhs != rhs));
                return val;
            }

            Opcode::Pow => {

                // Rust pow for i64 requires a u32 so we attempt to convert it to a u32
                // if it fails a PANIC!
                let rhs_converted = rhs.to_f64();
                
                return Float::from(lhs.pow(rhs_converted));
            }

            Opcode::Mod => {
                return lhs % rhs;
            }

            Opcode::Lsh => {

                let rhs_converted = match u64::try_from(rhs.to_integer().unwrap()) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"lsh\": {}", e);
                    }
                };

                let lhs_converted = match u64::try_from(lhs.to_integer().unwrap()) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"lsh\": {}", e);
                    }
                };

                let mut val = Float::new(FLOAT_PRECISION);
                val.assign(Integer::from(lhs_converted << rhs_converted));
                return val;
            }

            Opcode::Rsh => {

                let rhs_converted = match u64::try_from(rhs.to_integer().unwrap()) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"rhs\": {}", e);
                    }
                };

                let lhs_converted = match u64::try_from(lhs.to_integer().unwrap()) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"rhs\": {}", e);
                    }
                };
                
                let mut val = Float::new(FLOAT_PRECISION);
                val.assign(Integer::from(lhs_converted >> rhs_converted));
                return val;
            }

            Opcode::BwXor => {
                return Float::with_val(FLOAT_PRECISION, lhs.to_integer().unwrap() ^ rhs.to_integer().unwrap() );
            }

            Opcode::BwOr => {
                return Float::with_val(FLOAT_PRECISION, lhs.to_integer().unwrap() | rhs.to_integer().unwrap() );
            }

            Opcode::BwAnd => {
                return Float::with_val(FLOAT_PRECISION, lhs.to_integer().unwrap() & rhs.to_integer().unwrap() );
            }

            Opcode::Or => {

                if lhs > 0 || rhs > 0 {
                    let mut val = Float::new(FLOAT_PRECISION);
                    val.assign(Integer::from(1));
                    return val;
                }
                let mut val = Float::new(FLOAT_PRECISION);
                val.assign(Integer::from(0));
                return val;
            }

            Opcode::And => {
                
                if lhs > 0 && rhs > 0{
                    let mut val = Float::new(FLOAT_PRECISION);
                    val.assign(Integer::from(1));
                    return val;
                }
                let mut val = Float::new(FLOAT_PRECISION);
                val.assign(Integer::from(0));
                return val;
            }
        }
    }
}