#[macro_use]
extern crate tracing;

use crate::janki_app::JankiApp;
use eframe::NativeOptions;

mod janki_app;

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    eframe::run_native(
        "Janki",
        NativeOptions::default(),
        Box::new(|_cc| Box::new(JankiApp::new())),
    );
}
