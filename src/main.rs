mod storage;
mod task;

use std::env;
use std::io::ErrorKind;

use storage::{load_tasks, save_tasks};
use task::Task;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage");
        println!(" add <task description>");
        println!(" list");
        println!(" check <task number>");
        println!(" update <task number> <new description>");
        println!(" delete <task number>");
        return Ok(());
    }

    let command = &args[1];

    let mut tasks = match load_tasks() {
        Ok(tasks) => tasks,
        Err(e) if e.kind() == ErrorKind::NotFound => Vec::new(),
        Err(e) => return Err(e.into()),
    };

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Please provide a task description.");
                return Ok(());
            }
            let description = args[2..].join(" ");
            tasks.push(Task::new(description));

            save_tasks(&tasks)?;
            println!("Task added successfully.");
        }

        "list" => {
            if tasks.is_empty() {
                println!("No tasks found.");
            } else {
                for (index, task) in tasks.iter().enumerate() {
                    println!(
                        "{}. [{}] {}",
                        index + 1,
                        if task.completed { "x" } else { " " },
                        task.description
                    );
                }
            }
        }

        "check" => {
            if args.len() < 3 {
                println!("Please provide a task to check.");
                return Ok(());
            }
            let task_no = &args[2];
            let number: usize = task_no.parse()?;

            if number == 0 {
                println!("Task number must be greater than 0.");
                return Ok(());
            }

            let index = parse_index(task_no, tasks.len())?;
            tasks[index].completed = true;
            save_tasks(&tasks)?;
            println!("Task checked.");
        }

        "update" => {
            if args.len() < 4 {
                println!("Please provide a task number and new description.");
                return Ok(());
            }
            let task_no = &args[2];
            let description = args[3..].join(" ");
            let number: usize = task_no.parse()?;

            if number == 0 {
                println!("Task number must be greater than 0.");
                return Ok(());
            }

            let index = parse_index(task_no, tasks.len())?;
            tasks[index].description = description;
            save_tasks(&tasks)?;
            println!("Task updated successfully.");
        }

        "delete" => {
            if args.len() < 3 {
                println!("Please provide a task to delete.");
                return Ok(());
            }
            let task_no = &args[2];
            let number: usize = task_no.parse()?;

            if number == 0 {
                println!("Task number must be greater than 0.");
                return Ok(());
            }

            let index = parse_index(task_no, tasks.len())?;
            tasks.remove(index);
            save_tasks(&tasks)?;
            println!("Task deleted successfully.");
        }

        _ => {
            println!("Unknown command.");
        }
    }

    Ok(())
}

fn parse_index(arg: &str, len: usize) -> Result<usize, String> {
    let number: usize = arg.parse().map_err(|_| "Invalid number")?;

    if number == 0 || number > len {
        return Err("Task number out of range".into());
    }

    Ok(number - 1)
}