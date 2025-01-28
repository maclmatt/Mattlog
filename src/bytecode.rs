use crate::term::Term;

#[derive(Debug)]
pub enum Bytecode {
    Call(String),
    Unify(Term),
    Allocate,
    Deallocate,
    Backtrack,
    Proceed,
}
