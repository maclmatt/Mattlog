pub mod parser;
mod database;
mod solver;
mod terms;
mod unification;

//use database::Database;
//use solver::solve;
use terms::Term;

#[allow(unused_imports)]
use parser::tree::{ TermKind, Clause, variable, atom, compound, conjunct, fact, rule };
use parser::parser::{ parse, parse_query };
use parser::solver::{ Database, Query, Partial };

fn main() {
    let input = "
        nth([X|Xs], 0, X).
        nth([S|Xs], N, Y) :- is(M, N - 1), nth(Xs, M, Y).
    ";

    let query_string = "
        nth([1, 2, 3, 4], 2, X).
    ";


    let clauses = parse(input).unwrap();
    println!("{:?}", clauses);
    let db = Database::new(clauses);

    let query_term = parse_query(query_string).unwrap();
    println!("{:?}", query_term);
    let query = Query::new(query_term);

    //if let Some(result) = solve(&query, &db) {
    //    println!("Solution: {:?}", result);
    //} else {
    //    println!("No solution found.");
    //}

    let input = "
        true_fact.
    ";

    let query_string = "
        true_fact.
    ";

    let clauses = parse(input).unwrap();
    println!("{:?}", clauses);
    let db = Database::new(clauses);

    let query_term = parse_query(query_string).unwrap();
    println!("{:?}", query_term);
    let query = Query::new(query_term);

    match query.solve_from(&db, 0) {
        Some(partial) => println!("Result: \x1b[32m{}\x1b[0m", partial.result),
        None => println!("Result: \x1b[31mfalse\x1b[0m"),
    }
}
