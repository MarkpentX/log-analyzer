use crate::filters::{filter_by_counts, filter_by_type};

mod filters;
mod log_counter;

pub fn search(contents: String, filter_type: Option<String>){
    match filter_type {
        Some(filter_type) => {
            filter_by_type(contents, filter_type);
        }
        None => {
            filter_by_counts(contents);
        }
    }
}

