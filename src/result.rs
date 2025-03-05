use crate::terms::Term;
use crate::unification::Substitution; // This is the correct source

pub fn get_result(query_text: &str, solution: Option<Substitution>) -> String {
    match solution {
        Some(subs) => {
            if subs.is_empty() {
                format!("{} => true", query_text)
            } else {
                let mut result = String::new();
                for (var, term) in subs.iter() {
                    let value = format_term(term, &subs);
                    result.push_str(&format!("{} = {}\n", var, value));
                }
                format!("{} => {}\ntrue", query_text, result.trim())
            }
        }
        None => format!("{} => false", query_text),
    }
}

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
            let mut current = tail.as_ref();

            while let Term::List(next_head, next_tail) = current {
                items.push(format_term(next_head, subs));
                current = next_tail.as_ref();
            }

            if let Term::EmptyList = current {
                format!("[{}]", items.join(", "))
            } else {
                // This covers the case of improper lists (e.g., [a|b])
                let tail_str = format_term(current, subs);
                format!("[{} | {}]", items.join(", "), tail_str)
            }
        }
        Term::EmptyList => "[]".to_string(),
        Term::Conjunct(left, right) => {
            format!("{}, {}", format_term(left, subs), format_term(right, subs))
        }
    }
}

