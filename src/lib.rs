mod log_counter;

use crate::log_counter::{LogLevel, LogStats};

pub fn search(contents: String){
    let mut stats = LogStats::new();
    for line in contents.lines(){
        let result = filter_log(line);

        match result.as_str() {
            "INFO" => stats.add(LogLevel::Info),
            "WARN" => stats.add(LogLevel::Warn),
            "ERROR" => stats.add(LogLevel::Error),
            _ => {}
        }
    }

    stats.print();
}

pub fn filter_log(log: &str) -> String{
    let result = log.split(" ").skip(2).collect::<Vec<&str>>()[0];
    let mut item = result.to_string();
    item.remove(0);
    item.remove(item.len() - 1);
    item
}