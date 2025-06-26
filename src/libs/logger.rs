use String;
use std::env;
use tracing_logfmt;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt};
use tracing_subscriber::{fmt, prelude::*};

pub struct Logger;

impl Logger {
    /// Initializes the logger with either "json" or "text" format.
    pub fn init(format: String) {
        let filter_layer =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

        let format_string = env::var("LOG_FORMAT").unwrap_or_else(|_| format);
        let log_format = format_string.as_str();

        match log_format {
            "json" => {
                tracing_subscriber::registry()
                    .with(filter_layer)
                    .with(fmt::layer().json().with_level(true).with_line_number(true))
                    .init();
            }
            _ => {
                tracing_subscriber::registry()
                    .with(filter_layer)
                    .with(tracing_logfmt::layer())
                    .with(fmt::layer().with_level(true).with_line_number(true))
                    .init();
            }
        }
    }
}
