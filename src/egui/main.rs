use crate::janki_app::JankiApp;
use eframe::NativeOptions;

mod janki_app;

fn main() {
    eframe::run_native(
        "Janki",
        NativeOptions::default(),
        Box::new(|_cc| Box::new(JankiApp::new())),
    );
}
