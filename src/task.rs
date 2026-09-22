
#[derive(Clone,Debug)]
pub enum Status {
    Open,
    Closed
}

#[derive(Clone,Debug)]
pub struct Task {
    pub title: String,
    pub status: Status,
    pub priority: i32,
    pub tags: Vec<String>,
    pub body: String
}

pub fn load_tasks() -> Vec<Task> {
    todo!("load_tasks");
    let tasks = Vec::new();

    return tasks;
}

pub fn print_tasks(tasks: Vec<Option<Task>>) {
    todo!("print_tasks");
}
