use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::json::JsonEncoder;
use log4rs::{Config, Handle};

fn make_config(lvl: LevelFilter) -> Config {
    let json_stdout = ConsoleAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build();

    Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(json_stdout)))
        .build(Root::builder().appender("stdout").build(lvl))
        .unwrap()
}

pub struct Logger {
    handle: Handle,
}

pub fn init() -> Logger {
    Logger {
        handle: log4rs::init_config(make_config(LevelFilter::Info)).unwrap(),
    }
}

impl Logger {
    pub fn set_level(&self, lvl: LevelFilter) {
        self.handle.set_config(make_config(lvl));
    }
}
