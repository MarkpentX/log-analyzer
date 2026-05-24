use std::ops::Add;
use crate::log_counter::{LogLevel, LogStats};

pub fn filter_by_counts(contents: String){
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

pub fn filter_by_type(contents: String, filter_type: String) {
    let mut results = Vec::new();

    for line in contents.lines() {
        let result = filter_log(line);

        if result == filter_type {
            results.push(line);
        }
    }

    if results.is_empty() {
        println!("No logs found for level: {}", filter_type);
    } else {
        println!("Filtered logs [{}]:", filter_type);
        println!("{}", results.join("\n"));
    }
}

fn filter_log(log: &str) -> String{
    let result = log.split(" ").skip(2).collect::<Vec<&str>>()[0];
    let mut item = result.to_string();
    item.remove(0);
    item.remove(item.len() - 1);
    item
}
