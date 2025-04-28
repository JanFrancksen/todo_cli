mod db;
mod task;

use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use db::{load_tasks, save_tasks};
use task::{Priority, Task};
use uuid::Uuid;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        description: String,
        #[arg(short, long)]
        due: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
        #[arg(short, long, value_enum, default_value_t = Priority::Medium)]
        priority: Priority,
    },
    List,
    Done {
        id: Uuid,
    },
    Delete {
        id: Uuid,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add {
            description,
            due,
            tags,
            priority,
        } => {
            let due_date = match due {
                Some(due_str) => Some(
                    DateTime::parse_from_rfc3339(&due_str)
                        .unwrap()
                        .with_timezone(&Local),
                ),
                None => None,
            };
            tasks.push(Task {
                id: Uuid::new_v4(),
                description,
                done: false,
                due_date,
                tags,
                priority,
            });
            save_tasks(&tasks).unwrap();
            println!("Task added");
        }
        Commands::List => {
            for task in &tasks {
                println!(
                    "{} [{}] - {} ({:?})",
                    task.id,
                    if task.done { "x" } else { " " },
                    task.description,
                    task.priority
                );
                if let Some(due) = &task.due_date {
                    println!("    Due: {}", due);
                }
                if !task.tags.is_empty() {
                    println!("    Tags: {}", task.tags.join(", "));
                }
            }
        }
        Commands::Done { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.done = true;
                save_tasks(&tasks).unwrap();
                println!("Task marked as done!");
            }
        }
        Commands::Delete { id } => {
            tasks.retain(|t| t.id != id);
            save_tasks(&tasks).unwrap();
            println!("Task deleted!");
        }
    }
}
