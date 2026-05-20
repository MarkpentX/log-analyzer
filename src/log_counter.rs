use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

pub struct LogStats {
    pub counts: HashMap<LogLevel, usize>,
}

impl LogStats {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn add(&mut self, log_level: LogLevel){
        *self.counts.entry(log_level).or_insert(0) += 1
    }

    pub fn print(&self) {
        for (level, count) in &self.counts {
            println!("{:?}: {}", level, count);
        }
    }
}