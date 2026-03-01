use std::fs;
use std::io::{self};

use crate::task::Task;

const FILE_PATH: &str = "tasks.txt";

pub fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let mut content = String::new();

    for task in tasks {
        let line = format!(
            "{}|{}\n", 
            if task.completed { 1 } else { 0 }, 
            task.description
        );
        content.push_str(&line);
    }
    
    fs::write(FILE_PATH, content)?;
    Ok(())
}

pub fn load_tasks() -> io::Result<Vec<Task>> {
    let content = fs::read_to_string(FILE_PATH)?;
    let mut tasks = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() == 2 {
            let completed = parts[0] == "1";
            let description = parts[1].to_string();
            
            tasks.push(Task {
                description,
                completed,
            });
        }
    }

    Ok(tasks)
}