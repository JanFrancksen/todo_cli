use crate::task::Task;
use std::fs;
use std::io::{self};

const FILE_PATH: &str = "todo.json";

pub fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap()
}

pub fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write(FILE_PATH, data)
}
