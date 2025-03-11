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

                    ui.add_space(5.0);
                    ui.group(|ui| {
                        ui.add_sized(
                            [320.0, 400.0],
                            egui::TextEdit::multiline(&mut self.rules_text)
                                .font(egui::TextStyle::Monospace),
                        );
                    });

                    ui.add_space(10.0);

                    ui.horizontal_centered(|ui| {
                        if ui.button("Load Rules").clicked() {
                            let file_path = "program.pl";
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
                });

                ui.separator();

                // Right side: Query section (Input & History)
                ui.vertical(|ui| {
                    ui.heading("Query & Results");
                    ui.add_space(5.0);

                    ui.label("Current Query:");
                    ui.add(egui::TextEdit::singleline(&mut self.query_text));

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        if ui.button("Run Query").clicked() {
                            if let Some(ref db) = self.db {
                                match parse_query(&self.query_text) {
                                    Ok(parsed_query) => {
                                        let query = Term::from_tree_term(parsed_query);
                                        let query_expr = Expression::from_term(query);

                                        let mut stack = BacktrackingStack::new();
                                        let mut counter = 0;
                                        let solution = solver::solve(&query_expr, db, &mut stack, &mut counter);

                                        let final_solution = match solution {
                                            Some(sol) => Some(sol),
                                            _none => {
                                                while let Some(choice) = stack.pop() {
                                                    let retry_solution = solver::solve(&Expression::Term(choice.alternatives[0].clone()), db, &mut stack, &mut counter);
                                                    if retry_solution.is_some() {
                                                        break;
                                                    }
                                                }
                                                None
                                            }
                                        };

                                        let result = result::get_result(&self.query_text, final_solution);

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
                    });

                    ui.add_space(10.0);

                    ui.heading("Query History:");
                    egui::ScrollArea::vertical().max_height(380.0).show(ui, |ui| {
                        for entry in &self.query_history {
                            ui.group(|ui| {
                                ui.label(entry);
                            });
                        }
                    });
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
