

pub mod ast;

pub mod env;

#[macro_use] 
extern crate lalrpop_util;

lalrpop_mod!(pub micron); // synthesized by LALRPOP

/*
struct Vm {

    eval_stack: Vec<i32>
}

impl Vm {

    fn new() -> Self {
        Self {
            eval_stack: Vec::new()
        }
    }

    fn walk(&mut self, expr: ast::Expr) {
    
        match expr {
            ast::Expr::Number(n) => {
                //println!("{}", n);

                self.eval_stack.push(n);
            }

            ast::Expr::Op(lhs, op, rhs) => {

                self.walk(*lhs);
                self.walk(*rhs);

                //println!("{:?}", op);

                let l = self.eval_stack.pop().unwrap();
                let r = self.eval_stack.pop().unwrap();
                let result : i32 = match op {
                    ast::Opcode::Mul => {
                        l * r
                    }
                    ast::Opcode::Div => {
                        l / r
                    }
                    ast::Opcode::Add => {
                        l + r
                    }
                    ast::Opcode::Sub => {
                        l - r
                    }
                };

                self.eval_stack.push(result);
            }
        }
    }

    fn eval(&mut self, expr: ast::Expr) {

        self.walk(expr);

        println!("Result : {}", self.eval_stack.pop().unwrap());
    }
}
*/

fn main() {
    println!("Hello, world!");

    //let mut vm = Vm::new();

   // let expr = micron::ExprParser::new()
   //     .parse("(22 * 44) * (3 + (66 * 2) )")
   //     .unwrap();
//
   // vm.eval(*expr);

    let mut env = env::Environment::new();

    let statements = micron::ProgramParser::new()
        .parse("let a = 3 + 2;\n\
                let b = 4 + 4;\n\
                let a_1 = 3;")
        .unwrap();

    for x in statements {

        env.evaluate_statement(*x);
    } 

}
