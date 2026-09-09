use std::env;
use tatr::*;

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
