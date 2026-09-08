use std::env;
use std::fs;

fn usage() {
    println!("Usage: tatr [OPTION]...");
    println!("\tinit\tInitialise tasks directory");
    println!("\thelp\tShow this help message");
    println!("\tversion\tShow current version");
}

fn init() {
    match fs::create_dir("./tasks") {
        Ok(_) => println!("Successfully created tasks directory"),
        Err(e) => eprintln!("ERROR: Could not create tasks directory:\n\t{}", e)
    }
}

fn version() {
    unreachable!("TODO: Not implemented yet");
    // println!("tatr {}");
    // println!("\tCommit {}");
}

fn main() {
    let mut args = env::args();
    
    if args.len() == 1 {
        usage();
        return;
    }

    _ = args.next();  // arv[0]
    match args.next().unwrap().as_str() {
        "init" => init(),
        "help" => usage(),
        "version" => version(),
        _ => usage()
    }
}
