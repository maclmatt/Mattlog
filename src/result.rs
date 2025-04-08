use crate::terms::Term;
use crate::unification::Substitution;

use std::time::Duration;

pub fn get_result(query_text: &str, solution: Option<Substitution>, duration: Duration) -> String {
    match solution {
        Some(subs) => {
            let query_vars = extract_query_vars(query_text);

            let results: Vec<String> = query_vars
                .iter()
                .filter_map(|var| {
                    subs.get(var).map(|term| format!("{} = {}", var, format_term(term, &subs)))
                })
                .collect();

            if results.is_empty() {
                format!("{} => true", query_text)
            } else if duration.as_millis() > 10 {
                format!("{} => {}      |   Solve time: {:?}ms", query_text, results.join(", "), duration.as_millis())
            } else {
                format!("{} => {}", query_text, results.join(", "))
            }
        }
        _none => format!("{} => false", query_text),
    }
}

// Extract variables from query text
fn extract_query_vars(query: &str) -> Vec<String> {
    let re = regex::Regex::new(r"\b[A-Z_][A-Za-z0-9_]*\b").unwrap();
    re.find_iter(query).map(|mat| mat.as_str().to_string()).collect()
}

// Ensure your format_term stays the same, as previously defined:
fn format_term(term: &Term, subs: &Substitution) -> String {
    match term {
        Term::Integer(n) => n.to_string(),
        Term::Constant(c) => c.clone(),
        Term::Variable(v) => {
            if let Some(resolved_term) = subs.get(v) {
                format_term(resolved_term, subs)
            } else {
                v.clone()
            }
        }
        Term::Compound(name, args) => {
            let args_str: Vec<String> = args.iter().map(|arg| format_term(arg, subs)).collect();
            format!("{}({})", name, args_str.join(", "))
        }
        Term::List(head, tail) => {
            let mut items = vec![format_term(head, subs)];
            let mut current_tail = tail.as_ref();
            while let Term::List(head, next_tail) = current_tail {
                items.push(format_term(head, subs));
                current_tail = next_tail.as_ref();
            }
            if let Term::EmptyList = current_tail {
                format!("[{}]", items.join(", "))
            } else {
                format!("[{} | {}]", items.join(", "), format_term(current_tail, subs))
            }
        }
        Term::EmptyList => "[]".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terms::Term;
    use crate::unification::Substitution;
    use std::time::Duration;

    #[test]
    fn test_result_with_variable_binding() {
        let query = "?- X = 1.";
        let mut subs = Substitution::new();
        subs.extend("X".to_string(), Term::Integer(1));

        let result = get_result(query, Some(subs), Duration::from_millis(5));
        assert_eq!(result, "?- X = 1. => X = 1");
    }

    #[test]
    fn test_result_with_multiple_bindings_and_long_duration() {
        let query = "?- X = 1, Y = foo.";
        let mut subs = Substitution::new();
        subs.extend("X".to_string(), Term::Integer(1));
        subs.extend("Y".to_string(), Term::Constant("foo".to_string()));

        let result = get_result(query, Some(subs), Duration::from_millis(50));
        assert!(
            result.starts_with("?- X = 1, Y = foo. => X = 1, Y = foo"),
            "Expected formatted output with bindings"
        );
        assert!(result.contains("Solve time: 50ms"), "Expected duration in output");
    }

    #[test]
    fn test_result_with_true_output_when_no_vars() {
        let query = "?- foo(bar).";
        let subs = Substitution::new();

        let result = get_result(query, Some(subs), Duration::from_millis(2));
        assert_eq!(result, "?- foo(bar). => true");
    }

    #[test]
    fn test_result_with_no_solution() {
        let query = "?- X = 1.";
        let result = get_result(query, None, Duration::from_millis(0));
        assert_eq!(result, "?- X = 1. => false");
    }

    #[test]
    fn test_result_with_list_binding() {
        let query = "?- X = [1, 2, 3].";
        let mut subs = Substitution::new();
        let list = Term::List(
            Box::new(Term::Integer(1)),
            Box::new(Term::List(
                Box::new(Term::Integer(2)),
                Box::new(Term::List(
                    Box::new(Term::Integer(3)),
                    Box::new(Term::EmptyList),
                )),
            )),
        );
        subs.extend("X".to_string(), list);

        let result = get_result(query, Some(subs), Duration::from_millis(3));
        assert!(result.contains("[1, 2, 3]"));
    }
}
