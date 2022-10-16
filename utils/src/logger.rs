use tracing::Level;
use tracing_subscriber::{
    filter, fmt,
    prelude::*,
    reload::{self, Handle},
    Registry,
};

#[derive(Debug, Clone)]
pub struct Logger {
    handle: Handle<filter::LevelFilter, Registry>,
}

pub fn init(lvl: Level) -> Logger {
    let (filtered_layer, reload_handle) = reload::Layer::new(lvl.into());

    tracing_subscriber::registry()
        .with(filtered_layer)
        .with(
            fmt::Layer::default()
                .json()
                .flatten_event(true)
                .with_line_number(true)
                .with_file(true)
                .with_thread_ids(true)
                .with_target(true)
                .with_span_list(true)
                .with_current_span(false)
                .with_span_events(fmt::format::FmtSpan::NEW | fmt::format::FmtSpan::CLOSE),
        )
        .init();

    Logger {
        handle: reload_handle,
    }
}

impl Logger {
    pub fn set_level(&self, lvl: Level) {
        self.handle.reload(Some(lvl)).unwrap();
    }
}
