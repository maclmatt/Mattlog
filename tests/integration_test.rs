#[cfg(test)]
mod tests {
    use prolog_interpreter::{parser::parse_program, interpreter::Interpreter};

    #[test]
    fn test_prolog_program() {
        let ast = parse_program("parent(john, mary).");
        let mut interpreter = Interpreter::new();
        // Execute the AST or its bytecode equivalent
    }
}
