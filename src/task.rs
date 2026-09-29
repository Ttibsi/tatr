use std::fs;
use std::env;

use crate::*;

#[derive(Clone,Debug)]
pub enum Status {
    Open,
    Closed
}

impl Status {
    fn from(line: String) -> Option<Status> {
        if line.contains("OPEN") { return Some(Status::Open); }
        if line.contains("CLOSED") { return Some(Status::Closed); }
        return None;
    }
}

#[derive(Clone,Debug)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub status: Status,
    pub priority: i32,
    pub tags: Vec<String>,
    pub body: String
}

impl Task {
    fn from(id: String, t: String, s: Status, prio: i32, tags: Vec<String>, b: String) -> Task {
        return Task {
            id: id, 
            title: t,
            status: s,
            priority: prio,
            tags: tags,
            body: b
        };
    }

    fn from_file(contents: Vec<String>, folder: String) -> Option<Task> {
        if contents.is_empty() { return None; }

        let title = contents[0].split_at(1).1.to_string();
        let status_str = Status::from(<String as Clone>::clone(&contents[2])).unwrap();
        let priority = contents[3].split_at(11).1.trim().parse::<i32>().unwrap();
        let tags = contents[4].split_at(6).1.trim().split(',').map(|s| s.to_string()).collect();
        let body = contents[5..].join("\n").trim().to_string();

        return Some(Task::from(folder, title, status_str, priority, tags, body));
    }

    fn display(&self) {
        println!("{}:1:{}", self.id, self.title);
    }
}

pub fn load_tasks() -> Vec<Task> {
    let mut tasks = Vec::new();

    let pwd = env::current_dir().expect("Could not get CWD");
    let start = find_tasks_dir(&pwd).expect("Cannot find tasks directory");

    for entry in fs::read_dir(start).unwrap() {
        let folder = entry.unwrap().path().display().to_string();
        let task_file = format!("{}/TASK.md", folder);

        let contents = read_lines(task_file);
        if let Some(task) = Task::from_file(contents, folder) {
            tasks.push(task);
        }
    }

    return tasks;
}

pub fn print_tasks(tasks: Vec<Option<Task>>) {
    for task in tasks {
        if let Some(t) = task {
            t.display();
        }
    }
}
