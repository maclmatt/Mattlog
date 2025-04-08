use crate::environment::Environment;
use crate::terms::Term;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terms::Term;
    use crate::environment::Environment;

    #[test]
    fn test_stack_push_and_pop() {
        let mut stack = BacktrackingStack::new();
        let env = Environment::new();
        let alt = vec![Term::Integer(42)];

        let choice = ChoicePoint {
            env,
            alternatives: alt.clone(),
        };

        stack.push(choice);
        let popped = stack.pop();

        assert!(popped.is_some());
        let popped_choice = popped.unwrap();
        assert_eq!(popped_choice.alternatives, alt);
    }

    #[test]
    fn test_stack_empty_pop() {
        let mut stack = BacktrackingStack::new();
        assert!(stack.pop().is_none());
    }

    #[test]
    fn test_stack_lifo_order() {
        let mut stack = BacktrackingStack::new();

        let cp1 = ChoicePoint {
            env: Environment::new(),
            alternatives: vec![Term::Integer(1)],
        };
        let cp2 = ChoicePoint {
            env: Environment::new(),
            alternatives: vec![Term::Integer(2)],
        };

        stack.push(cp1);
        stack.push(cp2);

        let last = stack.pop().unwrap();
        assert_eq!(last.alternatives[0], Term::Integer(2));

        let first = stack.pop().unwrap();
        assert_eq!(first.alternatives[0], Term::Integer(1));

        assert!(stack.pop().is_none());
    }
}
