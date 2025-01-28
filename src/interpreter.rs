use crate::bytecode::Bytecode;
use crate::environment::Environment;
use crate::backtracking::BacktrackingStack;

pub struct Interpreter {
    env: Environment,
    stack: BacktrackingStack,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: Environment::new(),
            stack: BacktrackingStack::new(),
        }
    }

    pub fn execute(&mut self, code: Vec<Bytecode>) {
        for instruction in code {
            match instruction {
                Bytecode::Call(predicate) => {
                    println!("Calling {}", predicate);
                    // Handle predicate lookup and execution.
                }
                Bytecode::Unify(term) => {
                    println!("Unifying {:?}", term);
                    // Perform unification.
                }
                Bytecode::Allocate => {
                    println!("Allocating environment");
                    // Set up local environment.
                }
                Bytecode::Deallocate => {
                    println!("Deallocating environment");
                    // Cleanup environment.
                }
                Bytecode::Backtrack => {
                    println!("Backtracking");
                    if let Some(choice) = self.stack.pop() {
                        self.env = choice.env;
                    }
                }
                Bytecode::Proceed => {
                    println!("Proceeding");
                    // Continue to the next instruction.
                }
            }
        }
    }
}

#[cfg(test)]
mod interpreter_tests {
    use super::*;

    #[test]
    fn test_interpreter_execution() {
        let mut interpreter = Interpreter::new();
        let program = vec![
            Bytecode::Allocate,
            Bytecode::Call("parent".into()),
            Bytecode::Deallocate,
            Bytecode::Proceed,
        ];
        interpreter.execute(program);
    }
}
