use crate::environment::Environment;
use crate::term::Term;

pub struct ChoicePoint {
    pub env: Environment,
    pub alternatives: Vec<Term>,
}

pub struct BacktrackingStack {
    stack: Vec<ChoicePoint>,
}

impl BacktrackingStack {
    pub fn new() -> Self {
        BacktrackingStack { stack: Vec::new() }
    }

    pub fn push(&mut self, choice: ChoicePoint) {
        self.stack.push(choice);
    }

    pub fn pop(&mut self) -> Option<ChoicePoint> {
        self.stack.pop()
    }
}
