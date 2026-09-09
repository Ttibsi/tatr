use std::fs;

pub fn usage() {
    println!("Usage: tatr [OPTION]...");
    println!("\tinit\tInitialise tasks directory");
    println!("\thelp\tShow this help message");
    println!("\tversion\tShow current version");
}

pub fn init() {
    match fs::create_dir("./tasks") {
        Ok(_) => println!("Successfully created tasks directory"),
        Err(e) => eprintln!("ERROR: Could not create tasks directory:\n\t{}", e)
    }
}

pub fn version() {
    unreachable!("TODO: Not implemented yet");
    // println!("tatr {}");
    // println!("\tCommit {}");
}
