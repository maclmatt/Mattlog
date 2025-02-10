pub mod parser;
mod database;
mod solver;
mod terms;
mod unification;

use database::Database;
use solver::solve;
use terms::Term;

use crate::solver::solve;
use crate::terms::Query;

#[allow(unused_imports)]
use parser::tree::{ TermKind, Clause, variable, atom, compound, conjunct, fact, rule };
use parser::parser::{ parse, parse_query };

fn main() {
    // Define a simple Prolog program with a fact.
    let input = "
        parent(alice, bob).
    ";

    // Define a query that checks if Alice is Bob's parent.
    let query_string = "
        parent(alice, bob).
    ";

    // Parse the Prolog program into structured clauses.
    let clauses = parse(input).expect("Failed to parse program.");
    println!("Parsed clauses: {:?}", clauses);

    // Create a database from parsed clauses.
    let db = Database::new(clauses);

    // Parse the query into a structured term.
    let query_term = parse_query(query_string).expect("Failed to parse query.");
    println!("Parsed query: {:?}", query_term);

    // Create a query object.
    let query = Query::new(query_term);

    // Try solving the query using `solve`.
    if let Some(result) = solve(&query, &db) {
        println!("Solution: {:?}", result);
    } else {
        println!("No solution found.");
    }

    // Try solving the query step-by-step.
    match query.solve_from(&db, 0) {
        Some(partial) => println!("Result: \x1b[32m{}\x1b[0m", partial.result),
        None => println!("Result: \x1b[31mfalse\x1b[0m"),
    }
}