use std::collections::BTreeMap;
use std::env;

use tatr::*;

fn main() {
    let mut args: BTreeMap<String, String> = parse_args(env::args());
    if args.is_empty() {
        usage();
        return;
    }

    if let Some((key, _)) = args.pop_first() {
        if key == "init" {
            init();
        } else if key == "help" {
            usage();
        } else if key == "version" {
            version();
        } else {
            usage();
        }
    }

}
