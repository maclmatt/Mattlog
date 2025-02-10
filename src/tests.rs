#[cfg(test)]
mod tests {
    use super::*;
    use crate::terms::{Term, Clause};

    #[test]
    fn test_unification() {
        let mut subs = HashMap::new();
        let term1 = Term::Var("X".to_string());
        let term2 = Term::Integer(42);

        assert!(unify(&term1, &term2, &mut subs));
        assert_eq!(subs.get("X"), Some(&Term::Integer(42)));
    }

    #[test]
    fn test_query_solving() {
        let db = Database::new(vec![
            Clause::Fact(Term::Compound("true_fact".to_string(), vec![])),
        ]);

        let query = Term::Compound("true_fact".to_string(), vec![]);
        assert!(solve(&query, &db).is_some());
    }
}
