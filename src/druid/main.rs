use druid::{AppLauncher, Data, PlatformError, Widget, WindowDesc};
use janki::{
    file_storage::NamedFileStorage,
    game::{default_sag, AnkiGame},
    item::Fact,
};

#[derive(Debug, Clone, Data)]
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

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder);
    let mut anki = AnkiGame::new("./janki_db.json".into(), default_sag())?;
    let data = JankiState::Viewing {
        show_defs: false,
        show_only_eligible: true,
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}

fn ui_builder() -> impl Widget<JankiState> {
    todo!()
}
