// use dioxus::prelude::Props;
use janki::{
    file_storage::NamedFileStorage,
    item::{Fact, ItemGuard},
};

//copied from the egui example
// #[derive(Props)]
pub enum JankiState<'a> {
    Testing {
        current_text: String,
        current_fact: ItemGuard<'a, NamedFileStorage>,
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
    // Csv {
    //     file_name: String,
    //     overwrite_existing: bool,
    // },
}
