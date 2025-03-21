use crate::terms::Term;
use crate::unification::Substitution;

use std::time::{Duration, Instant};

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
