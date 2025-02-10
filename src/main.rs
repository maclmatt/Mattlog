mod database;
mod parser;
mod terms;
mod unification;
mod solver;

use database::Database;
use parser::parser::{parse, parse_query};
use solver::solve;
use terms::{Clause, Term, Expression};
use std::io;

fn main() {
    // Sample Prolog-like program as input
    let input = "
        a.
    ";

    let query_string = "a.";

    // Parse input into clauses
    let clauses = parse(input).expect("Failed to parse input.");
    let db = Database::new(clauses.into_iter().map(Clause::from_tree_clause).collect());

    // Parse query
    let query_term = parse_query(query_string).expect("Failed to parse query.");
    let query = Term::from_tree_term(query_term);

    println!("Database: {:?}", db);
    println!("Query: {:?}", query);

    // Convert query from Term to Expression
    let query_expr = Expression::from_term(query);  
    // Solve query
    if let Some(solution) = solve(&query_expr, &db) {
        println!("Solution: {:?}", solution);
    } else {
        println!("No solution found.");
    }
    //Boolean response (TODO: Working progress)
    if let Some(substitutions) = solve(&query_expr, &db) {
        if substitutions.is_empty() {
            println!("true"); // Query matched exactly (no variables)
        } else {
            println!("true"); // A valid substitution was found
        }
    } else {
        println!("false"); // No valid unification possible
    }

    // Interactive mode
    loop {
        println!("\nEnter a query (or type 'exit' to quit):");

        let mut user_query = String::new();
        io::stdin().read_line(&mut user_query).expect("Failed to read input");

        let user_query = user_query.trim();
        if user_query.eq_ignore_ascii_case("exit") {
            break;
        }

        match parse_query(user_query) {
            Ok(parsed_query) => {
                let query = Term::from_tree_term(parsed_query);
                if let Some(solution) = solve(&query_expr, &db) {
                    println!("Solution: {:?}", solution);
                } else {
                    println!("No solution found.");
                }
                //Boolean response (TODO: Working progress)
                if let Some(substitutions) = solve(&query_expr, &db) {
                    if substitutions.is_empty() {
                        println!("true"); // Query matched exactly (no variables)
                    } else {
                        println!("true"); // A valid substitution was found
                    }
                } else {
                    println!("false"); // No valid unification possible
                }
            }
            Err(_) => println!("Invalid query format."),
        }
    }
}
