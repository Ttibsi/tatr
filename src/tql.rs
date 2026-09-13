#[derive(Debug)]
enum Token {
    Tag(String),
    Number(i32),
    Obracket,
    Cbracket,
    Not,
    Any,
    Priority,
    Eq,
    Ne,
    Le,
    Lt,
    Ge,
    Gt
}

fn tokenize(args: String, debug: bool)  -> Vec<Token> {
    let mut tokens = Vec::new();
    if debug {
        println!("Tokens:");
    }

    let mut buf = String::new();
    for c in args.chars() {
        if c.is_whitespace() {
            if buf.parse::<i32>().is_ok() {
                tokens.push(Token::Number(buf.parse::<i32>().unwrap()));
                buf.clear();
                continue;
            } else if buf.chars().nth(0).unwrap() == ':' {
                tokens.push(Token::Tag(buf.strip_prefix(":").unwrap().to_string()));
                buf.clear();
                continue;
            }
        }

        buf.push(c);

        if buf == "not" {
            tokens.push(Token::Not);
            buf.clear();
        } else if buf == "any" {
            tokens.push(Token::Any);
            buf.clear();
        } else if buf == "priority" {
            tokens.push(Token::Priority);
            buf.clear();
        } else if buf == "eq" {
            tokens.push(Token::Eq);
            buf.clear();
        } else if buf == "ne" {
            tokens.push(Token::Ne);
            buf.clear();
        } else if buf == "lt" {
            tokens.push(Token::Lt);
            buf.clear();
        } else if buf == "le" {
            tokens.push(Token::Le);
            buf.clear();
        } else if buf == "gt" {
            tokens.push(Token::Gt);
            buf.clear();
        } else if buf == "ge" {
            tokens.push(Token::Ge);
            buf.clear();
        } else if buf == "[" {
            tokens.push(Token::Obracket);
            buf.clear();
        } else if buf == "]" {
            tokens.push(Token::Cbracket);
            buf.clear();
        }
    }

    if debug {
        for elem in &tokens {
            println!("\t{:?}", elem);
        }
    }

    return tokens;
}

pub fn parse_tql(args: String, debug: bool) {
    let tokens: Vec<Token> = tokenize(args, debug);
}
