use std::env;
use std::collections::BTreeMap;

pub fn parse_args(args: env::Args) -> BTreeMap<String, String> {
    let arg_list: Vec<String> = args.skip(1).collect();
    let mut parsed: BTreeMap<String, String> = BTreeMap::new();
    let mut key: Option<String> = None;
    for elem in arg_list {
        let first_char = elem.chars().nth(0).unwrap();
        if first_char == '-' {
            continue;
        }

        if let Some(ref k) = key {
            parsed.insert(k.to_string(), elem);
            key = None;
        } else {
            parsed.insert(elem, "true".to_string());
        }
    }

    return parsed;
}
