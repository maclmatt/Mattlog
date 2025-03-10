use crate::database::Database;
use crate::terms::{Clause, Term, Expression};
use crate::unification::{Substitution, unify};
use crate::backtracking::{BacktrackingStack, ChoicePoint};
use crate::environment::Environment;
use std::collections::HashMap;

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
            for (head, body) in &matching_clauses[1..] {
                stack.push(ChoicePoint {
                    env: Environment::new(),
                    alternatives: vec![Term::Compound(name.clone(), args.clone())],
                });
            }

            let (first_head, first_body) = &matching_clauses[0];
            let mut local_subs = subs.clone();

            if unify(term, first_head, &mut local_subs) {
                // Explicitly apply substitutions to both head and body terms
                let resolved_head = first_head.apply(&local_subs);
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

// Finds an appropriate replacement variable for reassignment
fn find_replacement_var(var: &str, retained_vars: &[String]) -> Option<String> {
    retained_vars.iter().find(|&&ref v| v != var).cloned()
}

// Extracts all variables present in a term recursively
fn extract_variables(term: &Term) -> Vec<String> {
    let mut vars = Vec::new();
    match term {
        Term::Variable(var) => vars.push(var.clone()),
        Term::Compound(_, args) => {
            for arg in args {
                vars.extend(extract_variables(arg));
            }
        }
        Term::List(head, tail) => {
            vars.extend(extract_variables(head));
            vars.extend(extract_variables(tail));
        }
        _ => {}
    }
    vars
}

// Reassigns values before removing unnecessary bindings for all recursive rules
fn reassign_and_cleanup_bindings(subs: &mut Substitution, term: &Term) {
    let retained_vars: Vec<String> = extract_variables(term);
    println!("Retained variables: {:?}", retained_vars);

    let mut removed_vars: Vec<String> = vec![];
    let mut reassignments: Vec<(String, Term)> = vec![];

    for (var, value) in subs.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>() {
        if !retained_vars.contains(&var) {
            // Find a logical replacement if it exists
            if let Some(replacement_var) = find_replacement_var(&var, &retained_vars) {
                println!("Reassigning {} to {}", var, replacement_var);
                reassignments.push((replacement_var, value));
            }
            removed_vars.push(var);
        }
    }

    // Apply reassignments before removing variables
    for (new_var, value) in reassignments {
        subs.extend(new_var, value);
    }

    println!("Variables to remove: {:?}", removed_vars);
    for var in removed_vars {
        println!("Removing variable: {}", var);
        subs.remove(&var);
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
