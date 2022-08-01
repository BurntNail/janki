mod state;

use crate::state::JankiState;
use dioxus::prelude::*;
use janki::{
    file_storage::NamedFileStorage,
    game::{default_sag, AnkiGame, GiveItemGuards},
};
use tracing::{error, info};

type AnkiG = AnkiGame<NamedFileStorage, GiveItemGuards>;

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let game = use_state(&cx, || {
        AnkiGame::<_, GiveItemGuards>::new(NamedFileStorage::from("./janki_db.json"), default_sag())
            .unwrap_or_else(|err| {
                error!("Error init-ing AG: {err}");
                std::process::exit(1);
            })
    });

    let sidebar = rsx!(
        ul {
            style: "list-style-type: none",
            li {
                button {onclick: move |_| info!("A"), "A"}
            }
            li {
                button {onclick: move |_| info!("B"), "B"}
            }
            li {
                button {onclick: move |_| info!("C"), "C"}
            }

        }
    );

    rsx!(cx, rsx!(sidebar))
}
