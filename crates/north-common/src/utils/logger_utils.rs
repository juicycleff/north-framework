use yansi::Paint;

use log::{Level, LevelFilter, Metadata, Record};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Logger as LoggerRs, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

pub fn print_format(name: &str, value: &str) {
    println!(
        "=> {}: {}",
        Paint::green(name).bold(),
        Paint::default(value).bold()
    );
}

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init_logger() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[North] {h({d(%Y-%m-%d %H:%M:%S %z)})} {l} [{t}] - {m}{n}",
        )))
        .build();

    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[North] {h({d(%Y-%m-%d %H:%M:%S %z)})} {l} [{t}] - {m}{n}",
        )))
        .build("log/requests.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(LoggerRs::builder().build("arangors_lite::response", LevelFilter::Info))
        .logger(LoggerRs::builder().build("hyper", LevelFilter::Warn))
        .logger(LoggerRs::builder().build("reqwest", LevelFilter::Warn))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .unwrap();
    if log4rs::init_config(config).is_ok() {};
}
