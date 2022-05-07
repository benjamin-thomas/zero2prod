use std::io::{sink, stdout};

use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn init() {
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");

    // RUST_LOG=info unless specified otherwise
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let show_log = std::env::var("LOG").unwrap_or_else(|_| "0".to_string()) == "1";

    // trait for `Subscriber` exposed by `tracing_subscriber`
    if show_log {
        let formatting_layer = BunyanFormattingLayer::new("zero2prod".to_string(), stdout);
        let subscriber = Registry::default()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(formatting_layer);

        // Define which subscriber should be used to process spans.
        set_global_default(subscriber).expect("Failed to set subscriber");
    } else {
        let formatting_layer = BunyanFormattingLayer::new("zero2prod".to_string(), sink);
        let subscriber = Registry::default()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(formatting_layer);

        // Define which subscriber should be used to process spans.
        set_global_default(subscriber).expect("Failed to set subscriber");
    };
}
