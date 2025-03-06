mod database;
mod parser;
mod terms;
mod unification;
mod solver;
mod result;
mod backtracking;
mod environment;

use database::Database;
use parser::parser::{parse, parse_query};
use solver::solve;
use terms::{Clause, Term, Expression};
use eframe::{egui, App, Frame};
use std::fs;
use backtracking::BacktrackingStack;

struct PrologApp {
    rules_text: String,
    query_text: String,
    query_history: Vec<String>,
    db: Option<Database>,
}

impl Default for PrologApp {
    fn default() -> Self {
        Self {
            rules_text: String::new(),
            query_text: String::new(),
            query_history: Vec::new(),
            db: None,
        }
    }
}

impl App for PrologApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Left side: Rules section
                ui.vertical(|ui| {
                    ui.heading("Rule Tree");
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(300.0, 400.0), // Fixed size: 300 wide, 400 tall
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                            ui.text_edit_multiline(&mut self.rules_text);
                        },
                    );
                    
                    if ui.button("Load Rules from File").clicked() {
                        let file_path = "program.pl"; // Simplified — replace if needed
                        match fs::read_to_string(file_path) {
                            Ok(content) => self.rules_text = content,
                            Err(err) => self.query_history.push(format!("Failed to load file: {}", err)),
                        }
                    }

                    if ui.button("Parse Rules").clicked() {
                        match parse(&self.rules_text) {
                            Ok(tree_clauses) => {
                                let clauses = tree_clauses
                                    .into_iter()
                                    .map(Clause::from_tree_clause)
                                    .collect();
                                self.db = Some(Database::new(clauses));
                                self.query_history.push("Rules parsed successfully.".to_string());
                            }
                            Err(_) => {
                                self.query_history.push("Failed to parse rules.".to_string());
                            }
                        }
                    }
                });

                // Right side: Query section (Input & History)
                ui.vertical(|ui| {
                    ui.heading("Query & Results");

                    ui.label("Current Query");
                    ui.text_edit_singleline(&mut self.query_text);

                    if ui.button("Run Query").clicked() {
                        if let Some(ref db) = self.db {
                            match parse_query(&self.query_text) {
                                Ok(parsed_query) => {
                                    let query = Term::from_tree_term(parsed_query);
                                    let query_expr = Expression::from_term(query);
                    
                                    // ✅ Create a BacktrackingStack for the solver
                                    let mut stack = BacktrackingStack::new();
                    
                                    // ✅ Call the solver with backtracking support
                                    let solution = solver::solve(&query_expr, db, &mut stack);
                    
                                    // ✅ If no solution is found, backtrack and try alternatives
                                    let final_solution = match solution {
                                        Some(sol) => Some(sol),
                                        None => {
                                            while let Some(choice) = stack.pop() {
                                                let retry_solution = solver::solve(&Expression::Term(choice.alternatives[0].clone()), db, &mut stack);
                                                if retry_solution.is_some() {
                                                    break;
                                                }
                                            }
                                            None
                                        }
                                    };
                    
                                    // ✅ Format the result
                                    let result = result::get_result(&self.query_text, final_solution);
                    
                                    // ✅ Add to history
                                    self.query_history.push(result);
                                }
                                Err(_) => {
                                    self.query_history.push(format!("{} => Invalid query format", self.query_text));
                                }
                            }
                        } else {
                            self.query_history.push("No rules loaded. Please parse rules first.".to_string());
                        }
                    }
                    

                    ui.separator();

                    ui.label("Query History:");
                    for entry in &self.query_history {
                        ui.label(entry);
                    }
                });
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mini Prolog",
        options,
        Box::new(|_cc| Box::<PrologApp>::default()),
    )
}
