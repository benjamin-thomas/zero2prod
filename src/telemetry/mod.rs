use std::io::{sink, stdout};

use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

// trait for `Subscriber` exposed by `tracing_subscriber`
fn get_subscriber<T>(log_identifier: &str, output: T) -> impl Subscriber + Sync + Send
where
    // T implements the `MakeWriter` trait for all choices of the lifetime parameter `'a`
    // Check out https://doc.rust-lang.org/nomicon/hrtb.html for more details.
    T: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // RUST_LOG=info unless specified otherwise
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer = BunyanFormattingLayer::new(log_identifier.to_string(), output);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    set_global_default(subscriber).expect("Failed to set subscriber");
}

pub fn init(log_identifier: &str) {
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");

    let show_log = std::env::var("LOG").unwrap_or_else(|_| "0".to_string()) == "1";
    if show_log {
        let subscriber = get_subscriber(log_identifier, stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(log_identifier, sink);
        init_subscriber(subscriber);
    };
}
