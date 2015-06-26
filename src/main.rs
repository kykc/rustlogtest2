#[macro_use] extern crate log;
extern crate env_logger;
use ::std::fs::{File, OpenOptions};
use ::std::io::{BufWriter, Write};
use log::{LogRecord};

fn write_to_log(record: &LogRecord) {
    let mut options = OpenOptions::new();
    // We want to write to our file as well as append new data to it.
    options.write(true).append(true).create(true);

    let file = match options.open("test_file.txt") {
        Ok(file) => file,
        Err(..)  => panic!("room"),
    };

    let mut writer = BufWriter::new(&file);
    // Then we write to the file. write_all() calls flush() after the write as well.
    //writer.write_all(message);
    //writer.write_all(b"\n");
    writer.write_fmt(record.args);
}

use log::{LogRecord, LogLevel, LogMetadata, SetLoggerError, LogLevelFilter};

struct SimpleLogger {
    level: LogLevel,
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            //println!("{} - {}", record.level(), record.args());
            write_to_log(record);
        }
    }
}

pub fn init(level: LogLevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(level);
        Box::new(SimpleLogger{level: level.to_log_level().unwrap_or(LogLevel::Error)})
    })
}

fn main() {

    init(LogLevelFilter::Trace).unwrap();

    println!("Hello, world!");
    info!("info");
    trace!("trace");
    debug!("debug");
    error!("error");
}
