/*
:project: telegram-onedrive
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

mod cleaner;
mod formatter;
pub mod indenter;
mod visitor;

use crate::env::ENV;
use formatter::EventFormatter;
use indenter::FileIndenterLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn trace_registor() {
    LogTracer::init().unwrap();

    let trace_level = &ENV.get().unwrap().trace_level;

    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .event_format(EventFormatter);

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(FileIndenterLayer)
        .with(EnvFilter::new(trace_level).add_directive("sqlx=error".parse().unwrap()))
        .init();

    cleaner::run();
}
