use crate::bytecode::Bytecode;
use crate::environment::Environment;
use crate::backtracking::{BacktrackingStack, ChoicePoint};
use crate::terms::Term;

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
                    // Push a choice point if alternatives exist.
                    if let Some(alternatives) = self.get_alternatives(&predicate) {
                        self.stack.push(ChoicePoint {
                            env: self.env.clone(),
                            alternatives,
                        });
                    }
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
                    println!("Backtracking...");
                    if let Some(choice) = self.stack.pop() {
                        self.env = choice.env;
                        println!("Restoring environment and retrying alternatives...");
                        for alternative in choice.alternatives {
                            self.execute(vec![Bytecode::Call(alternative.to_string())]);
                        }
                    }
                }
                Bytecode::Proceed => {
                    println!("Proceeding");
                    // Continue to the next instruction.
                }
            }
        }
    }

    fn get_alternatives(&self, predicate: &str) -> Option<Vec<Term>> {
        // Retrieve alternative rules for a predicate.
        None
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
