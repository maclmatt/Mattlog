# Mattlog: A Mini-Prolog Interpreter in Rust

A lightweight Prolog interpreter written entirely in Rust, built from scratch to support core logic programming features such as unification, rule evaluation, backtracking, and built-in predicates, all with a graphical user interface powered by [`egui`](https://github.com/emilk/egui).

---

## Features

- Parsing of Prolog rules and queries
- Unification of variables, constants, compound terms, lists, and integers
- Custom backtracking engine
- Built-in predicates (`append`, `member`, `length`, `between`, `succ`, `reverse`, `sort`)
- GUI for writing and executing Prolog queries interactively
- Execution time measurement and debug output
- Modular design with extensibility in mind

---

## Architecture Overview

The interpreter is structured into modular components:

- `parser` — Converts Prolog syntax into an Abstract Syntax Tree (AST)
- `terms` — Core data structures (terms, lists, integers, compound terms)
- `unification` — Variable binding and substitution system
- `solver` — Query execution engine with recursive evaluation
- `backtracking` — Stack-based handling of alternatives and choice points
- `builtins` — Implementation of built-in predicates
- `result` — Formatting and timing of query results
- `gui (main.rs)` — Graphical interface using `eframe`/`egui`

---

## Usage

```bash
cargo run
```

Then paste or load Prolog rules and enter a query into the GUI. Example:

```prolog
parent(john, mary).
parent(mary, susan).

ancestor(X, Y) :- parent(X, Y).
ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).
```

Query:
```prolog
?- ancestor(john, susan).
```

---

## Parser Note

This project uses the **Prolog parser from [conlog](https://github.com/transistorfet/conlog)** (by [@transistorfet](https://github.com/transistorfet)) as its foundation. The parser was integrated and **modified slightly** to support proper operator precedence and ensure compatibility with the AST-based evaluation in this interpreter. Full credit to the original author for the base.

---

## Testing

Tests have been written to cover:

- Unification and substitution logic
- Backtracking and recursion
- Built-in predicate behavior
- Parser correctness

---

## License

MIT License.

---

## Acknowledgements

- [conlog](https://github.com/transistorfet/conlog) – for the original parser
- [egui](https://github.com/emilk/egui) – for the GUI framework
```