use std::path::Path;
use time::OffsetDateTime;
use std::fs::File;
use std::io::{self, BufRead};

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

// Copied from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> Option<Vec<String>> where P: AsRef<Path> {
    if let Ok(file) = File::open(filename) {
        let mut lines = Vec::new();
        for line in io::BufReader::new(file).lines().map_while(Result::ok) {
            lines.push(line);
        }

        return Some(lines);
    } else {
        return None;
    }
}
