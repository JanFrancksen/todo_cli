use chrono::{DateTime, Local};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub done: bool,
    pub due_date: Option<DateTime<Local>>,
    pub tags: Vec<String>,
    pub priority: Priority,
}
