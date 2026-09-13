use std::collections::VecDeque;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::vec::Vec;

use crate::*;

pub fn usage() {
    println!("Usage: tatr [OPTION]...");
    println!("\tinit\tInitialise tasks directory");
    println!("\thelp\tShow this help message");
    println!("\tversion\tShow current version");
}

pub fn init() {
    match fs::create_dir("./tasks") {
        Ok(_) => println!("Successfully created tasks directory"),
        Err(e) => eprintln!("ERROR: Could not create tasks directory:\n\t{}", e),
    }
}

pub fn version() {
    unreachable!("TODO: Not implemented yet");
    // println!("tatr {}");
    // println!("\tCommit {}");
}

pub fn new_cmd(args: &mut VecDeque<Arg>) {
    if has_key(args, "--help".to_string()) {
        println!("Usage: tatr new [OPTION]...");
        println!("OPTIONS:");
        println!("\t-p <int> Specify priority (default 100)");
        println!("\t-s <string> Task ID optional suffix");
        println!("\t-t <string> Add a tag to the task");
        println!("\t\tNOTE: This flag can be repeated ad infinitum");
        return;
    }

    let huid = generate_huid(get_value(args, "-s").as_deref());
    let pwd = env::current_dir().expect("Cound not get CWD");
    let tasks_dir = find_tasks_dir(&pwd).unwrap();
    let path = format!("{}/{}", tasks_dir, huid);
    let _ = fs::create_dir(&path);
    let mut f = match File::create(format!("{}/TASK.md", path)) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let title = args
        .pop_front()
        .unwrap_or(Arg {
            Key: "NEW TASK".to_string(),
            Value: None,
        })
        .Key;
    let priority = get_value(args, "-p").unwrap_or("100".to_string());
    let mut tags = Vec::new();
    for elem in args {
        if elem.Key == "-t" {
            tags.push(elem.Value.clone().unwrap().clone());
        }
    }

    let body = format!(
        "\
# {}

- STATUS: OPEN
- PRIORITY: {}
- TAGS: {}

",
        title,
        priority,
        tags.join(",")
    );

    match f.write_all(body.as_bytes()) {
        Ok(()) => (),
        Err(e) => eprintln!("Error writing to file: {}", e),
    }

    println!(
        "{}/TASK.md:1: OPEN [Priority: {}] [{}] {}",
        path,
        priority,
        tags.join(","),
        title
    );
}

pub fn ls_cmd(args: &mut VecDeque<Arg>) {
    let mut debug = false;
    if has_key(args, "-debug".to_string()) {
        pop_arg(args, "-debug");
        debug = true;
    }

    parse_tql(as_string(args), debug);
}
