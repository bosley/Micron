
extern crate micron_environment;
extern crate micron_ast;

use micron_ast::{ Statement };
use micron_environment::{ MicronEnv };

use crate::error::InterpreterError;
use crate::calc_expression::ExpressionCalculator;

pub struct MicronInterpreter <'a> {

    environment: &'a mut MicronEnv
}

impl <'a> MicronInterpreter <'a> {

    /// Create a new interpreter
    pub fn new(env: &'a mut MicronEnv) -> Self {
        Self {
            environment: env
        }
    }

    /// Interpret a statement
    pub fn interpret(&mut self, statement: Statement) -> Result<(), InterpreterError> {

        match statement {

            //  Variable numerical assignment
            //
            Statement::Assignment(variable, expression) => {

                // Create a calculator for the expression
                let mut calc = ExpressionCalculator::new(self.environment);

                // Attempt to calculate the expression
                match calc.evaluate_expression(*expression) {

                    Ok(obj) => {

                        // Attempt to set the variable to the calculated value
                        if let Err(e) =  self.environment.set_variable(&variable, obj, None) {
                            return Err(InterpreterError::EnvironmentError(e));
                        }

                        return Ok(())
                    }

                    Err(e) => {

                        return Err(e);
                    }
                }

            }

            // Evaluate raw expression
            //
            Statement::BareExpression(expression) => {

                // Create a calculator for the expression
                let mut calc = ExpressionCalculator::new(self.environment);

                // Attempt to calculate the expression
                match calc.evaluate_expression(*expression) {

                    Ok(obj) => {

                        println!("{:?}", obj);
                        return Ok(());
                    }

                    Err(e) => {
                        return Err(e);
                    }
                }
            }


        }
    }
}