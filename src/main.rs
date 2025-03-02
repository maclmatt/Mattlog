mod database;
mod parser;
mod terms;
mod unification;
mod solver;

use database::Database;
use parser::parser::{parse, parse_query};
use solver::solve;
use terms::{Clause, Term, Expression};

use eframe::{egui, NativeOptions};
use std::fs;

struct PrologApp {
    rules_text: String,
    current_query: String,
    result: String,
    query_history: Vec<(String, String)>,
    db: Option<Database>,
}

impl Default for PrologApp {
    fn default() -> Self {
        Self {
            rules_text: String::new(),
            current_query: String::new(),
            result: String::new(),
            query_history: vec![],
            db: None,
        }
    }
}

impl PrologApp {
    fn compile_rules(&mut self) {
        match parse(&self.rules_text) {
            Ok(parsed_clauses) => {
                let clauses: Vec<Clause> = parsed_clauses
                    .into_iter()
                    .map(Clause::from_tree_clause)
                    .collect();
                self.db = Some(Database::new(clauses));
                self.result = "Rules compiled successfully!".to_string();
            }
            Err(_) => {
                self.result = "Failed to parse rules.".to_string();
            }
        }
    }

    fn load_rules_from_file(&mut self) {
        if let Ok(content) = fs::read_to_string("program.pl") {
            self.rules_text = content;
        } else {
            self.result = "Failed to load 'program.pl'.".to_string();
        }
    }

    fn run_query(&mut self) {
        if self.db.is_none() {
            self.result = "Rules not compiled yet!".to_string();
            return;
        }

        if let Ok(parsed_query) = parse_query(&self.current_query) {
            let query = Term::from_tree_term(parsed_query);
            let query_expr = Expression::from_term(query);

            let db = self.db.as_ref().unwrap();
            match solve(&query_expr, db) {
                Some(solution) => {
                    if solution.is_empty() {
                        self.result = "true".to_string();
                    } else {
                        let mut result_strings = vec![];
                        for (var, term) in solution.iter() {
                            let resolved = solution.resolve(term);
                            let value = match resolved {
                                Term::Integer(n) => n.to_string(),
                                Term::Constant(c) => c.clone(),
                                _ => format!("{:?}", resolved),
                            };
                            result_strings.push(format!("{} = {}", var, value));
                        }
                        self.result = result_strings.join(", ");
                    }
                }
                None => {
                    self.result = "false".to_string();
                }
            }

            // Store query + result into history
            self.query_history
                .push((self.current_query.clone(), self.result.clone()));
        } else {
            self.result = "Invalid query format.".to_string();
        }
    }
}

impl eframe::App for PrologApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Rule Tree");
                    ui.add(
                        egui::TextEdit::multiline(&mut self.rules_text)
                            .desired_rows(15)
                            .desired_width(200.0),
                    );
                    
                    if ui.button("Compile Rules").clicked() {
                        self.compile_rules();
                    }

                    if ui.button("Load From File (program.pl)").clicked() {
                        self.load_rules_from_file();
                    }
                });

                ui.vertical(|ui| {
                    ui.label("Query History");
                    for (query, result) in &self.query_history {
                        ui.horizontal(|ui| {
                            if ui.button(query).clicked() {
                                self.current_query = query.clone();
                            }
                            ui.label(format!("=> {}", result));
                        });
                    }

                    if ui.button("Clear History").clicked() {
                        self.query_history.clear();
                    }
                });
            });

            ui.separator();

            ui.label("Current Query (editable)");
            ui.text_edit_singleline(&mut self.current_query);

            if ui.button("Run Query").clicked() {
                self.run_query();
            }

            ui.separator();

            ui.label("Results");
            ui.label(&self.result);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Mini Prolog GUI",
        options,
        Box::new(|_cc| Box::<PrologApp>::default()),
    )
}
