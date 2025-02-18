mod database;
mod parser;
mod terms;
mod unification;
mod solver;

use database::Database;
use parser::parser::{parse, parse_query};
use solver::solve;
use terms::{Clause, Term, Expression};
use std::fs;
use std::io::{self, Write};

fn main() {
    // Ask user for prolog filename
    print!("Enter the Prolog file name: ");
    io::stdout().flush().unwrap();

    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();

    let input = fs::read_to_string(filename).expect("Failed to read the file");
    
    // Parse input into clauses
    let clauses = parse(&input).expect("Failed to parse input.");
    let db = Database::new(clauses.into_iter().map(Clause::from_tree_clause).collect());

    println!("Loaded Prolog database from '{}'.", filename);

    // Interactive query loop
    loop {
        print!("\nEnter a query (or type 'exit' to quit): ");
        io::stdout().flush().unwrap(); // Flush output to ensure prompt appears

        let mut user_query = String::new();
        io::stdin().read_line(&mut user_query).expect("Failed to read input");

        let user_query = user_query.trim();
        if user_query.eq_ignore_ascii_case("exit") {
            println!("Exiting interactive mode.");
            break;
        }

        // Parse query
        match parse_query(user_query) {
            Ok(parsed_query) => {
                let query = Term::from_tree_term(parsed_query);
                let query_expr = Expression::from_term(query);

                // Solve the query
                match solve(&query_expr, &db) {
                    Some(solution) => {
                        if solution.is_empty() {
                            println!("true"); // Query matched exactly (no variables)
                        } else {
                            println!("Substitutions: {:?}", solution);
                        }

                        // If there's a substitution for "X", print it
                        if let Some(x_term) = solution.get("X") {
                            let resolved_x = solution.resolve(x_term);
                            match resolved_x {
                                Term::Integer(n) => println!("X = {}", n),
                                Term::Constant(c) => println!("X = {}", c),
                                _ => println!("X = {:?}", resolved_x), // For complex terms
                            }
                        }
                    }
                    None => {
                        println!("No solution found.");
                        println!("false");
                    }
                }
            }
            Err(_) => println!("Invalid query format. Try again."),
        }
    }
}