use std::path::Path;
use time::OffsetDateTime;

pub fn generate_huid(suffix: Option<&str>) -> String {
    let now = OffsetDateTime::now_utc();

    let base = format!(
        "{:04}{:02}{:02}-{:02}{:02}{:02}",
        now.year(),
        now.month() as u8,
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );

    match suffix {
        Some(s) if !s.is_empty() => format!("{}-{}", base, s),
        _ => base,
    }
}

pub fn find_tasks_dir(start_path: &Path) -> Option<String> {
    let mut current_path = start_path.canonicalize().ok()?;

    loop {
        let tasks_path = current_path.join("tasks");
        if tasks_path.is_dir() {
            return tasks_path
                .canonicalize()
                .ok()
                .map(|p| p.to_string_lossy().into_owned());
        }

        // Move to parent directory
        if let Some(parent) = current_path.parent() {
            if parent == current_path {
                break;
            }
            current_path = parent.to_path_buf();
        } else {
            break;
        }
    }

    None
}

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
