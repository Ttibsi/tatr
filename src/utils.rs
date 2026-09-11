use std::path::Path;

pub fn generate_huid(suffix: Option<String>) -> String {
    unreachable!("TODO: generate_huid");
}

pub fn find_tasks_dir(start_path: &Path) -> Option<String> {
    let mut current_path = start_path.canonicalize().ok()?;

    loop {
        let tasks_path = current_path.join("tasks");
        if tasks_path.is_dir() {
            return tasks_path.canonicalize().ok().map(|p| p.to_string_lossy().into_owned());
        }

        // Move to parent directory
        if let Some(parent) = current_path.parent() {
            if parent == current_path { break; }
            current_path = parent.to_path_buf();
        } else {
            break;
        }
    }

    None
}
