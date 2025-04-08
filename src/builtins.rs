use crate::unification::{Substitution, unify};
use crate::terms::Term;

pub fn builtin_append(args: &[Term]) -> Option<Substitution> {
    if args.len() != 3 {
        return None;
    }

    let list1 = &args[0];
    let list2 = &args[1];
    let result = &args[2];

    // Try converting terms to Vec representations
    match (list1.to_vec(), list2.to_vec(), result.to_vec()) {
        (Some(vec1), Some(vec2), _) => {
            // Both input lists known, unify combined with result
            let combined = [vec1, vec2].concat();
            let combined_term = Term::from_vec(&combined);
            let mut subs = Substitution::new();
            if unify(&args[2], &combined_term, &mut subs) {
                Some(subs)
            } else {
                None
            }
        }
        (Some(vec1), _none, Some(result_vec)) => {
            // First list and result known, calculate second list
            if result_vec.starts_with(&vec1) {
                let remaining = &result_vec[vec1.len()..];
                let mut subs = Substitution::new();
                if unify(&args[1], &Term::from_vec(remaining), &mut subs) {
                    Some(subs)
                } else {
                    None
                }
            } else {
                None
            }
        }
        (_none, Some(vec2), Some(result_vec)) => {
            // Second list and result known, unify to find first list
            if result_vec.ends_with(&vec2) {
                let prefix = &result_vec[..result_vec.len() - vec2.len()];
                let mut subs = Substitution::new();
                if unify(&args[0], &Term::from_vec(prefix), &mut subs) {
                    Some(subs)
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn builtin_member(args: &[Term]) -> Option<Substitution> {
    if args.len() != 2 { return None }  // Ensure correct arity
    let element = &args[0];      // Element to check for
    let list = &args[1];         // List to check
    match list {
        Term::List(head, tail) => {
            let mut subs = Substitution::new();
            // Check if element matches the head of the list
            if unify(element, head, &mut subs) {
                return Some(subs);
            }
            // Check recursively in the tail
            builtin_member(&[element.clone(), tail.as_ref().clone()])
        }
        Term::EmptyList => None, // Member of an empty list fails
        _ => None, // Not a valid list structure
    }
}

pub fn builtin_between(args: &[Term]) -> Option<Substitution> {
    if args.len() != 3 {
        return None; // Ensure correct arity
    }

    let low = match &args[0] {
        Term::Integer(n) => *n,
        _ => return None, // First argument must be an integer
    };

    let high = match &args[1] {
        Term::Integer(n) => *n,
        _ => return None, // Second argument must be an integer
    };

    let var = match &args[2] {
        Term::Variable(v) => v.clone(),
        _ => return None, // Third argument must be a variable
    };

    if low > high {
        return None; // Invalid range
    }

    // Collect all values as substitutions
    let results: Vec<String> = (low..=high)
        .map(|n| format!("{}", n)) // Convert integers to strings
        .collect();

    let result_str = results.join("; "); // Format output like "1; 2; 3; 4"

    let mut subs = Substitution::new();
    subs.extend(var, Term::Constant(result_str));

    Some(subs)
}

pub fn builtin_length(args: &[Term]) -> Option<Substitution> {
    if args.len() != 2 {
        return None; // Ensure correct arity
    }

    // Extract the list and expected length variable
    let mut count = 0;
    let mut current = &args[0];

    // Walk through the list to count elements
    while let Term::List(_head, tail) = current {
        count += 1;
        current = tail; // Move to the next element
    }

    // If it's an empty list, count remains 0
    if matches!(current, Term::EmptyList) {
        let var = match &args[1] {
            Term::Variable(v) => v.clone(),
            Term::Integer(n) if *n == count => return Some(Substitution::new()), // Matches given length
            _ => return None, // Second argument must be a variable or matching integer
        };

        let mut subs = Substitution::new();
        subs.extend(var, Term::Integer(count as i64));
        return Some(subs);
    }

    None // Not a valid list structure
}

pub fn builtin_reverse(args: &[Term]) -> Option<Substitution> {
    if args.len() != 2 {
        return None; // Ensure correct arity
    }

    let list = match &args[0] {
        Term::List(head, tail) => {
            let mut elements = vec![head.as_ref().clone()];
            let mut current_tail = tail.as_ref();
            while let Term::List(h, t) = current_tail {
                elements.push(h.as_ref().clone());
                current_tail = t.as_ref();
            }
            elements.reverse();
            elements
        }
        Term::EmptyList => vec![], // Reverse of an empty list is still empty
        _ => return None, // First argument must be a list
    };

    let var = match &args[1] {
        Term::Variable(v) => v.clone(),
        _ => return None, // Second argument must be a variable
    };

    let reversed_list = Term::list_from_vec(list);
    let mut subs = Substitution::new();
    subs.extend(var, reversed_list);
    Some(subs)
}

pub fn builtin_max(args: &[Term]) -> Option<Substitution> {
    if args.len() != 3 {
        return None; // Ensure correct arity
    }

    let left = match &args[0] {
        Term::Integer(n) => *n,
        _ => return None, // First argument must be an integer
    };

    let right = match &args[1] {
        Term::Integer(n) => *n,
        _ => return None, // Second argument must be an integer
    };

    let var = match &args[2] {
        Term::Variable(v) => v.clone(),
        _ => return None, // Third argument must be a variable
    };

    let max_val = std::cmp::max(left, right);
    let mut subs = Substitution::new();
    subs.extend(var, Term::Integer(max_val));

    Some(subs)
}

pub fn builtin_min(args: &[Term]) -> Option<Substitution> {
    if args.len() != 3 {
        return None;
    }

    let left = match &args[0] {
        Term::Integer(n) => *n,
        _ => return None,
    };

    let right = match &args[1] {
        Term::Integer(n) => *n,
        _ => return None,
    };

    let var = match &args[2] {
        Term::Variable(v) => v.clone(),
        _ => return None,
    };

    let min_val = std::cmp::min(left, right);
    let mut subs = Substitution::new();
    subs.extend(var, Term::Integer(min_val));

    Some(subs)
}

pub fn builtin_succ(args: &[Term]) -> Option<Substitution> {
    if args.len() != 2 {
        return None;
    }

    let number = match &args[0] {
        Term::Integer(n) => *n,
        _ => return None, // First argument must be an integer
    };

    let var = match &args[1] {
        Term::Variable(v) => v.clone(),
        _ => return None, // Second argument must be a variable
    };

    let mut subs = Substitution::new();
    subs.extend(var, Term::Integer(number + 1));

    Some(subs)
}

pub fn builtin_sort(args: &[Term]) -> Option<Substitution> {
    if args.len() != 2 {
        return None;
    }

    // Ensure first argument is a list
    let list_term = &args[0];
    let var = match &args[1] {
        Term::Variable(v) => v.clone(),
        _ => return None, // Second argument must be a variable
    };

    // Convert the input term into a Vec<Term>
    let vec = list_term.to_vec()?;

    // Only allow lists of integers for simplicity
    let mut values: Vec<_> = vec.iter()
        .filter_map(|t| {
            if let Term::Integer(i) = t {
                Some(*i)
            } else {
                None
            }
        })
        .collect();

    if values.len() != vec.len() {
        return None; // Input list must only contain integers
    }

    values.sort();

    let sorted_terms: Vec<Term> = values.into_iter().map(Term::Integer).collect();
    let sorted_term = Term::list_from_vec(sorted_terms);

    let mut subs = Substitution::new();
    subs.extend(var, sorted_term);

    Some(subs)

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terms::Term;

    #[test]
    fn test_builtin_append_basic() {
        let list1 = Term::list_from_vec(vec![Term::Integer(1), Term::Integer(2)]);
        let list2 = Term::list_from_vec(vec![Term::Integer(3)]);
        let result = Term::list_from_vec(vec![Term::Integer(1), Term::Integer(2), Term::Integer(3)]);

        let args = vec![list1, list2, result];
        let subs = builtin_append(&args);
        assert!(subs.is_some());
    }

    #[test]
    fn test_builtin_member_found() {
        let list = Term::list_from_vec(vec![
            Term::Integer(1),
            Term::Integer(2),
            Term::Integer(3),
        ]);
        let args = vec![Term::Integer(2), list];
        let subs = builtin_member(&args);
        assert!(subs.is_some());
    }

    #[test]
    fn test_builtin_member_not_found() {
        let list = Term::list_from_vec(vec![Term::Integer(1), Term::Integer(3)]);
        let args = vec![Term::Integer(2), list];
        let subs = builtin_member(&args);
        assert!(subs.is_none());
    }

    #[test]
    fn test_builtin_between_valid_range() {
        let args = vec![
            Term::Integer(1),
            Term::Integer(3),
            Term::Variable("X".into()),
        ];
        let subs = builtin_between(&args);
        assert!(subs.is_some());
        assert_eq!(subs.unwrap().get("X"), Some(&Term::Constant("1; 2; 3".into())));
    }

    #[test]
    fn test_builtin_length_correct() {
        let list = Term::list_from_vec(vec![Term::Integer(1), Term::Integer(2), Term::Integer(3)]);
        let args = vec![list, Term::Variable("N".into())];
        let subs = builtin_length(&args);
        assert!(subs.is_some());
        assert_eq!(subs.unwrap().get("N"), Some(&Term::Integer(3)));
    }

    #[test]
    fn test_builtin_reverse() {
        let list = Term::list_from_vec(vec![Term::Integer(1), Term::Integer(2)]);
        let expected = Term::list_from_vec(vec![Term::Integer(2), Term::Integer(1)]);
        let args = vec![list, Term::Variable("X".into())];
        let subs = builtin_reverse(&args);
        assert!(subs.is_some());
        assert_eq!(subs.unwrap().get("X"), Some(&expected));
    }

    #[test]
    fn test_builtin_max() {
        let args = vec![
            Term::Integer(3),
            Term::Integer(5),
            Term::Variable("M".into()),
        ];
        let subs = builtin_max(&args);
        assert!(subs.is_some());
        assert_eq!(subs.unwrap().get("M"), Some(&Term::Integer(5)));
    }

    #[test]
    fn test_builtin_min() {
        let args = vec![
            Term::Integer(3),
            Term::Integer(5),
            Term::Variable("M".into()),
        ];
        let subs = builtin_min(&args);
        assert!(subs.is_some());
        assert_eq!(subs.unwrap().get("M"), Some(&Term::Integer(3)));
    }

    #[test]
    fn test_builtin_succ() {
        let args = vec![
            Term::Integer(4),
            Term::Variable("X".into()),
        ];
        let subs = builtin_succ(&args);
        assert!(subs.is_some());
        assert_eq!(subs.unwrap().get("X"), Some(&Term::Integer(5)));
    }

    #[test]
    fn test_builtin_sort() {
        let list = Term::list_from_vec(vec![
            Term::Integer(3),
            Term::Integer(1),
            Term::Integer(2),
        ]);
        let expected = Term::list_from_vec(vec![
            Term::Integer(1),
            Term::Integer(2),
            Term::Integer(3),
        ]);
        let args = vec![list, Term::Variable("Sorted".into())];
        let subs = builtin_sort(&args);
        assert!(subs.is_some());
        assert_eq!(subs.unwrap().get("Sorted"), Some(&expected));
    }
}
