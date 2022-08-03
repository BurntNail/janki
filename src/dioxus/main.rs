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
    let game = use_ref(&cx, || {
        AnkiG::new(NamedFileStorage::from("./janki_db.json"), default_sag()).unwrap_or_else(|err| {
            error!("Error init-ing AG: {err}");
            std::process::exit(1);
        })
    });

    let big_s = "display: block; background-color: #444; color: white; text-align: center; padding: 14px 16px; text-decoration: none";

    let sidebar = rsx!(
        div {
            style: "width: 100vh; height: 100vw; background-color: #444",
            ul {
                style: "list-style-type: none; margin: 0; padding: 0; overflow: hidden; background-color: #333",
                li {
                    style: "float: left",
                    button {
                        style: format_args!("{big_s}"),
                        onclick: move |_| {
                            info!("Test");
                        },
                        "Test"
                    }
                }
                li {
                    style: "float: left",
                    button {
                        style: format_args!("{big_s}"),
                        onclick: move |_| {
                            info!("Add More");
                        },
                        "Add More"
                    }
                }
                li {
                    style: "float: left",
                    button {
                        style: format_args!("{big_s}"),
                        onclick: move |_| {
                            info!("View");
                        },
                        "View"
                    }
                }
            }
        }
    );

    rsx!(cx, rsx!(sidebar))
}
