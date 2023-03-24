use std::{fs::File, io::Write, path::Path};

use log::Log;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Level {
    Warn,
    Error,
    Info,
    Debug,
    Trace,
}

impl From<log::Level> for Level {
    fn from(value: log::Level) -> Self {
        match value {
            log::Level::Error => Self::Error,
            log::Level::Warn => Self::Warn,
            log::Level::Info => Self::Info,
            log::Level::Debug => Self::Debug,
            log::Level::Trace => Self::Trace,
        }
    }
}

impl From<Level> for log::Level {
    fn from(value: Level) -> Self {
        match value {
            Level::Error => log::Level::Error,
            Level::Warn => log::Level::Warn,
            Level::Info => log::Level::Info,
            Level::Debug => log::Level::Debug,
            Level::Trace => log::Level::Trace,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Record {
    pub level: Level,
    pub target: String,
    pub message: String,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

impl<'a> From<&log::Record<'a>> for Record {
    fn from(value: &log::Record) -> Self {
        Record {
            level: value.level().into(),
            target: value.target().to_owned(),
            message: format!("{}", value.args()),
            module_path: value.module_path().map(|s| s.to_owned()),
            file: value.file().map(|s| s.to_owned()),
            line: value.line().map(|s| s.to_owned()),
        }
    }
}

pub struct PipeLogger {
    file: File,
}

impl PipeLogger {
    pub fn new<P: AsRef<Path>>(log_file: P) -> Self {
        let mut new_file = File::options();
        PipeLogger {
            file: new_file.write(true).read(true).open(log_file).unwrap(),
        }
    }
}

impl Log for PipeLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let record: Record = record.into();
        ciborium::ser::into_writer(&record, &self.file).unwrap();
    }

    fn flush(&self) {
        (&mut &self.file).flush().unwrap()
    }
}
