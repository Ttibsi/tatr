use std::collections::VecDeque;
use std::env;

#[derive(Debug)]
pub struct Arg {
    pub Key: String,
    pub Value: Option<String>,
}

pub fn parse_args(args: env::Args) -> VecDeque<Arg> {
    let arg_list: Vec<String> = args.skip(1).collect();
    let mut parsed: VecDeque<Arg> = VecDeque::new();
    let mut key: Option<String> = None;
    for elem in arg_list {
        let first_char = elem.chars().nth(0).unwrap();
        if first_char == '-' {
            key = Some(elem);
            continue;
        }

        if let Some(ref k) = key {
            parsed.push_back(Arg {
                Key: k.to_string(),
                Value: Some(elem),
            });
            key = None;
        } else {
            parsed.push_back(Arg {
                Key: elem,
                Value: None,
            });
        }
    }

    println!("[INFO] Found: {:?}", parsed);
    return parsed;
}

pub fn has_key(args: &VecDeque<Arg>, key: String) -> bool {
    for elem in args {
        if elem.Key == key {
            return true;
        }
    }

    return false;
}

pub fn get_value(args: &VecDeque<Arg>, key: &str) -> Option<String> {
    for elem in args {
        if elem.Key == key {
            return elem.Value.clone();
        }
    }

    None
}
