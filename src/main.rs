use std::collections::VecDeque;
use std::env;

use tatr::*;

fn main() {
    let mut args: VecDeque<Arg> = parse_args(env::args());
    if args.is_empty() {
        usage();
        return;
    }

    let first = args.pop_front().unwrap();
    match first.Key.as_str() {
        "init" => init(),
        "help" => usage(),
        "version" => version(),
        "new" => new_cmd(&mut args),
        _ => usage(),
    };
}
