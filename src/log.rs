

const PREFIX_INFO: &str = "[Six502] ";
const PREFIX_ERROR: &str = "[Six502][ERROR]";

pub enum LogMode {
    Verbose,
    Silent,
}

enum LogLevel {
    Error,
    Info,
}

pub struct Log {
    mode: LogMode 
}

impl Log {

    #[cfg(test)]
    pub fn init() -> Self {
        Self {mode: LogMode::Verbose}
    }
    #[cfg(not(test))]
    pub fn init() -> Self {
        let mode = if let Ok(_) = std::env::var("RUST_LOG") {
            LogMode::Verbose
        } else {
            LogMode::Silent
        };
        Self {mode}
    }

    pub fn info(&self, data: &str) {
        self.log(data, LogLevel::Info);
    }
    pub fn error(&self, data: &str) {
        self.log(data, LogLevel::Error);
    }

    fn log(&self, data: &str, level: LogLevel) {
        match level {
            LogLevel::Error => 
                println!("{PREFIX_ERROR}{data}"),
            LogLevel::Info => 
                println!("{PREFIX_INFO}{data}"),
        }
    }
}

