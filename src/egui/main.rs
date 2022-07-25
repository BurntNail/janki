#[macro_use]
extern crate tracing;

use crate::janki_app::JankiApp;
use eframe::NativeOptions;
use tracing_subscriber::layer::SubscriberExt;

mod janki_app;

fn main() {
    //copied from https://fasterthanli.me/articles/when-rustc-explodes#tracing-deeper-with-debug-spans
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("janki_egui")
        .with_collector_endpoint("http://localhost:14268/api/traces")
        .install_simple()
        .unwrap();

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = tracing_subscriber::Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    eframe::run_native(
        "Janki",
        NativeOptions::default(),
        Box::new(|_cc| Box::new(JankiApp::new())),
    );
}
