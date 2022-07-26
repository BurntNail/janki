use eframe::{egui, egui::Context, Frame, Storage as EStorage};
use janki::{
    dummy_storage::{DummyStorage, DynStorage},
    game::{default_sag, AnkiGame, GiveFacts},
    item::Fact,
    storage::Storage as JStorage,
};
use std::time::Duration;
use tracing::Level;

pub enum JankiState {
    Testing {
        current_text: String,
        current_fact: Fact,
        was_eligible: bool,
    },
    Tested {
        fact: Fact,
        was_correct: bool,
    },
    AddingNew {
        term: String,
        def: String,
    },
    Viewing {
        show_defs: bool,
        show_only_eligible: bool,
    },
}

pub struct JankiApp {
    app: AnkiGame<DummyStorage, GiveFacts>,
    has_done_initial_read: bool,
    state: JankiState,
}

impl JankiApp {
    pub fn new() -> Self {
        Self {
            app: AnkiGame::new(DummyStorage::default(), default_sag()).unwrap(),
            state: JankiState::Viewing {
                show_defs: false,
                show_only_eligible: true,
            },
            has_done_initial_read: false,
        }
    }
}

impl eframe::App for JankiApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.has_done_initial_read {
            egui::SidePanel::left("left_side").show(ctx, |ui| {
                if ui.button("New Test").clicked() {
                    if let Some((f, was_eligible)) = self.app.get_new_fact() {
                        self.state = JankiState::Testing {
                            current_text: String::default(),
                            current_fact: f,
                            was_eligible,
                        };
                    }
                } else if ui.button("Add More").clicked() {
                    self.state = JankiState::AddingNew {
                        term: String::default(),
                        def: String::default(),
                    }
                } else if ui.button("View").clicked() {
                    self.state = JankiState::Viewing {
                        show_defs: false,
                        show_only_eligible: true,
                    }
                }

                ui.separator();

                if let JankiState::Viewing {
                    show_defs,
                    show_only_eligible,
                } = &mut self.state
                {
                    ui.checkbox(show_defs, "Show definitions: ");
                    ui.checkbox(show_only_eligible, "Show only eligible: ");
                    ui.separator();
                }

                ui.label(format!(
                    "Only {} Facts remaining this session!",
                    self.app.get_eligible_no(),
                ));
            });

            egui::CentralPanel::default().show(ctx, |ui| match &mut self.state {
                JankiState::Testing {
                    current_fact,
                    current_text,
                    was_eligible,
                } => {
                    if *was_eligible {
                        ui.label("Testing");
                    } else {
                        ui.label("EVEN MORE TESTING!");
                    }
                    ui.separator();

                    ui.label(format!("The term is: {}", current_fact.term));

                    ui.horizontal(|ui| {
                        ui.label("Please enter the definition: ");
                        ui.text_edit_singleline(current_text);
                    });

                    ui.separator();

                    if ui.button("Submit!").clicked() {
                        let was_correct = current_text.trim() == current_fact.definition;
                        self.app.finish_current_fact(Some(was_correct));

                        self.state = JankiState::Tested {
                            fact: current_fact.clone(),
                            was_correct,
                        };
                    }
                }
                JankiState::Tested { fact, was_correct } => {
                    if *was_correct {
                        ui.label("Correct!");
                    } else {
                        ui.label(format!("Wrong - it should've been {:?}", fact.definition));
                    }
                }
                JankiState::AddingNew { term, def } => {
                    ui.label("Add New Stuff");
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Enter a term: ");
                        ui.text_edit_singleline(term);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Enter a definition: ");
                        ui.text_edit_singleline(def);
                    });

                    if ui.button("Submit").clicked() {
                        self.app
                            .add_fact(Fact::new(term.to_string(), def.to_string()));
                        term.clear();
                        def.clear();
                    }
                }
                JankiState::Viewing {
                    show_defs,
                    show_only_eligible,
                } => {
                    let list = if *show_only_eligible {
                        self.app.get_eligible()
                    } else {
                        self.app.get_all_facts()
                    };

                    ui.label("Viewing Facts!");

                    ui.separator();

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        if !list.is_empty() {
                            list.into_iter()
                                .enumerate()
                                .for_each(|(index, f): (usize, Fact)| {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("Term - {}, ", f.term));
                                        if *show_defs {
                                            ui.label(format!("Definition - {}", f.definition));
                                        } else {
                                            ui.label("Definition Hidden!");
                                        }

                                        if ui.button("Delete fact").clicked() {
                                            self.app.delete_at_index(index);
                                        }
                                    });
                                });
                        } else {
                            ui.label("No facts");
                        }
                    });
                }
            });
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Loading...");
                ui.spinner();
            });
        }
    }

    fn save(&mut self, mut storage: &mut dyn EStorage) {
        if !matches!(self.state, JankiState::Testing { .. }) {
            if self.has_done_initial_read {
                self.app
                    .write_custom(&mut storage as &mut dyn JStorage<ErrorType = serde_json::Error>)
                    .expect("Failure to write to EGUI storage");
            } else {
                self.has_done_initial_read = true;
                trace!("Doing initial read");
                self.app
                    .read_custom(&storage as &dyn JStorage<ErrorType = serde_json::Error>)
                    .expect("Failure to read from EGUI storage");
            }
        }
    }

    fn on_exit(&mut self, _gl: &eframe::glow::Context) {
        self.app.exit();

        if cfg!(feature = "opentel") {
            opentelemetry::global::shutdown_tracer_provider();
        }
    }

    fn auto_save_interval(&self) -> Duration {
        if !self.has_done_initial_read {
            Duration::from_millis(20)
        } else {
            Duration::from_secs(30) //the normal behaviourr
        }
    }
}
