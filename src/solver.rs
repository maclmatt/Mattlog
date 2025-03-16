use crate::database::Database;
use crate::terms::{Clause, Term, Expression};
use crate::unification::{Substitution, unify};
use crate::backtracking::{BacktrackingStack, ChoicePoint};
use crate::environment::Environment;

pub fn solve(query: &Expression, db: &Database, stack: &mut BacktrackingStack, counter: &mut usize) -> Option<Substitution> {
    match query {
        Expression::Term(term) => solve_term(term, db, stack, counter),
        Expression::Conjunct(lhs, rhs) => {
            if let Some(lhs_subs) = solve(lhs, db, stack, counter) {
                let applied_rhs = rhs.apply(&lhs_subs);

                if let Some(rhs_subs) = solve(&applied_rhs, db, stack, counter) {
                    return lhs_subs.merge(&rhs_subs);
                }
            }
            while let Some(choice) = stack.pop() {
                println!("Backtracking...");
                return solve(&Expression::Term(choice.alternatives[0].clone()), db, stack, counter);
            }
            None
        }
    }
}

fn solve_term(term: &Term, db: &Database, stack: &mut BacktrackingStack, counter: &mut usize) -> Option<Substitution> {
    let mut subs = Substitution::new();

    if let Term::Compound(name, args) = term {
        if name == "is" && args.len() == 2 {
            println!("Evaluating: {:?} is {:?}", args[0], args[1]);
            if let Term::Variable(var) = &args[0] {
                if let Some(value) = evaluate_arithmetic(&args[1]) {
                    let mut result = Substitution::new();
                    result.extend(var.clone(), Term::Integer(value));
                    return Some(result);
                }
            }
            return None;
        }

        if RELATIONAL_OPERATORS.contains(&name.as_str()) && args.len() == 2 {
            return match evaluate_relation(name, &args[0], &args[1]) {
                Some(true) => Some(subs),
                _ => None,
            };            
        }

        if name == "append" && args.len() == 3 {
            return builtin_append(args); // Call your built-in append function
        }

        if name == "member" && args.len() == 2 {
            return builtin_member(args)
        }

        let mut matching_clauses = vec![];

        for clause in &db.clauses {
            let renamed_clause = rename_clause_variables(clause, *counter);
            match &renamed_clause {
                Clause::Fact(fact) if unify(term, fact, &mut subs) => {
                    return Some(subs);
                }
                Clause::Rule(head, body) if unify(term, head, &mut subs) => {
                    matching_clauses.push((head.clone(), body.clone()));
                }
                _ => {}
            }
            *counter += 1;  // Increment for the next clause
        }

        if !matching_clauses.is_empty() {
            for (_head, _body) in &matching_clauses[1..] {
                stack.push(ChoicePoint {
                    env: Environment::new(),
                    alternatives: vec![Term::Compound(name.clone(), args.clone())],
                });
            }

            let (first_head, first_body) = &matching_clauses[0];
            let mut local_subs = subs.clone();

            if unify(term, first_head, &mut local_subs) {
                // Explicitly apply substitutions to both head and body terms
                let _resolved_head = first_head.apply(&local_subs);
                let applied_body = first_body.apply(&local_subs);

                *counter += 1;  // Increment counter for recursive calls

                if let Some(body_subs) = solve(&applied_body, db, stack, counter) {
                    return local_subs.merge(&body_subs);
                }
            }

        }
    }

    None
}

fn rename_vars(term: &Term, suffix: usize) -> Term {
    match term {
        Term::Variable(var) => Term::Variable(format!("{}_{}", var, suffix)),
        Term::Compound(name, args) => Term::Compound(
            name.clone(),
            args.iter().map(|arg| rename_vars(arg, suffix)).collect(),
        ),
        Term::List(head, tail) => Term::List(
            Box::new(rename_vars(head, suffix)),
            Box::new(rename_vars(tail, suffix)),
        ),
        _ => term.clone(),
    }
}

fn rename_clause_variables(clause: &Clause, suffix: usize) -> Clause {
    match clause {
        Clause::Fact(term) => Clause::Fact(rename_vars(term, suffix)),
        Clause::Rule(head, body) => Clause::Rule(rename_vars(head, suffix), rename_expr(body, suffix)),
    }
}

fn rename_expr(expr: &Expression, suffix: usize) -> Expression {
    match expr {
        Expression::Term(term) => Expression::Term(rename_vars(term, suffix)),
        Expression::Conjunct(lhs, rhs) => Expression::Conjunct(
            Box::new(rename_expr(lhs, suffix)),
            Box::new(rename_expr(rhs, suffix)),
        ),
    }
}

// Arithmetic evaluation (for 'is')
fn evaluate_arithmetic(expr: &Term) -> Option<i64> {
    match expr {
        Term::Integer(n) => Some(*n),
        Term::Compound(op, args) if args.len() == 2 => {
            let left = evaluate_arithmetic(&args[0])?;
            let right = evaluate_arithmetic(&args[1])?;
            match op.as_str() {
                "+" => Some(left + right),
                "-" => Some(left - right),
                "*" => Some(left * right),
                "/" => if right != 0 { Some(left / right) } else { None },
                _ => None,
            }
        }
        _ => None,
    }
}

// Relation evaluation (for <, >, =<, etc.)
fn evaluate_relation(op: &str, left: &Term, right: &Term) -> Option<bool> {
    let left_value = evaluate_arithmetic(left)?;
    let right_value = evaluate_arithmetic(right)?;
    println!("Left and Right {} {}", left_value, right_value);

    match op {
        "<" => Some(left_value < right_value),
        ">" => Some(left_value > right_value),
        "=<" => Some(left_value <= right_value),
        ">=" => Some(left_value >= right_value),
        "=" => Some(left_value == right_value),
        "\\=" => Some(left_value != right_value),
        _ => None,
    }
}

// Operators we want to handle
const RELATIONAL_OPERATORS: [&str; 6] = ["<", ">", "=<", ">=", "=", "\\="];

fn builtin_append(args: &[Term]) -> Option<Substitution> {
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
        (Some(vec1), None, Some(result_vec)) => {
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
        (None, Some(vec2), Some(result_vec)) => {
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

fn builtin_member(args: &[Term]) -> Option<Substitution> {
    if args.len() != 2 {
        return None; // Ensure correct arity
    }

    let element = &args[0];  // The item we're checking for
    let list = &args[1];     // The list to check

    match list {
        Term::List(head, tail) => {
            let mut subs = Substitution::new();

            // First, check if element matches the head of the list
            if unify(element, head, &mut subs) {
                return Some(subs);
            }

            // Otherwise, check recursively in the tail
            builtin_member(&[element.clone(), tail.as_ref().clone()])
        }
        Term::EmptyList => None, // Member of an empty list fails
        _ => None, // Not a valid list structure
    }
}

