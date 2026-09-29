use std::collections::VecDeque;
use crate::Task;

#[derive(Debug, PartialEq)]
enum Token {
    Tag(String),
    Number(i32),
    Obracket,
    Cbracket,
    Or,
    And,
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

fn tokenize(args: String, debug: bool)  -> VecDeque<Token> {
    let mut tokens = VecDeque::new();

    let mut buf = String::new();
    for c in args.chars() {
        if c.is_whitespace() {
            if buf.parse::<i32>().is_ok() {
                tokens.push_back(Token::Number(buf.parse::<i32>().unwrap()));
                buf.clear();
                continue;
            } else if buf.chars().nth(0).unwrap() == ':' {
                tokens.push_back(Token::Tag(buf.strip_prefix(":").unwrap().to_string()));
                buf.clear();
                continue;
            }
        }

        buf.push(c);

        if buf == "not" {
            tokens.push_back(Token::Not);
            buf.clear();
        } else if buf == "any" {
            tokens.push_back(Token::Any);
            buf.clear();
        } else if buf == "or" {
            tokens.push_back(Token::Or);
            buf.clear();
        } else if buf == "and" {
            tokens.push_back(Token::And);
            buf.clear();
        } else if buf == "priority" {
            tokens.push_back(Token::Priority);
            buf.clear();
        } else if buf == "eq" {
            tokens.push_back(Token::Eq);
            buf.clear();
        } else if buf == "ne" {
            tokens.push_back(Token::Ne);
            buf.clear();
        } else if buf == "lt" {
            tokens.push_back(Token::Lt);
            buf.clear();
        } else if buf == "le" {
            tokens.push_back(Token::Le);
            buf.clear();
        } else if buf == "gt" {
            tokens.push_back(Token::Gt);
            buf.clear();
        } else if buf == "ge" {
            tokens.push_back(Token::Ge);
            buf.clear();
        } else if buf == "[" {
            tokens.push_back(Token::Obracket);
            buf.clear();
        } else if buf == "]" {
            tokens.push_back(Token::Cbracket);
            buf.clear();
        }
    }

    if tokens.is_empty() {
        tokens.push_back(Token::Any);
    }

    if debug {
        println!("Tokens:");
        for elem in &tokens {
            println!("\t{:?}", elem);
        }
        println!("");
    }

    return tokens;
}

#[derive(Clone, PartialEq)]
pub enum Opcode {
    Or,
    And,
    Any,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    Tag(String),
    Priority,
}

fn compile_primary(tokens: &mut VecDeque<Token>, query: &mut Vec<Opcode>) -> bool {
    if tokens.is_empty() { return false; }
    let tok = tokens.pop_front();

    match tok.unwrap() {
        Token::Tag(name) => {
            query.push(Opcode::Tag(name));
            return true;
        }
        Token::Obracket => {
            if compile_expr(tokens, query) { return false; }
            return true;
        },
        Token::Not => {
            query.push(Opcode::Ne);
            return true;
        },
        Token::Any => {
            query.push(Opcode::Any);
            return true;
        },
        Token::Priority => {
            query.push(Opcode::Priority);
            return true;
        }
        _ => { 
            unreachable!("Unknown token"); 
            return false;
        }
    }
}

fn compile_compare(tokens: &mut VecDeque<Token>, query: &mut Vec<Opcode>) -> bool {
    if !compile_primary(tokens, query) { return false; }
    loop {
        if tokens.is_empty() { break; }
        if let Some(tok) = tokens.front() {
            match tok {
                Token::Lt => {
                    if !compile_primary(tokens, query) { return false; }
                    tokens.pop_front();
                    query.push(Opcode::Lt);
                },
                Token::Le => {
                    if !compile_primary(tokens, query) { return false; }
                    tokens.pop_front();
                    query.push(Opcode::Le);
                },
                Token::Gt => {
                    if !compile_primary(tokens, query) { return false; }
                    tokens.pop_front();
                    query.push(Opcode::Gt);
                },
                Token::Ge => {
                    if !compile_primary(tokens, query) { return false; }
                    tokens.pop_front();
                    query.push(Opcode::Ge);
                },
                Token::Eq => {
                    if !compile_primary(tokens, query) { return false; }
                    tokens.pop_front();
                    query.push(Opcode::Eq);
                },
                Token::Ne => {
                    if !compile_primary(tokens, query) { return false; }
                    tokens.pop_front();
                    query.push(Opcode::Ne);
                },
                _ => {},
            }
            if !compile_primary(tokens, query) { return false; }
        }

        query.push(Opcode::And);
    }

    return true;
}

fn compile_and(tokens: &mut VecDeque<Token>, query: &mut Vec<Opcode>) -> bool {
    if !compile_compare(tokens, query) { return false; }
    loop {
        if tokens.is_empty() { break; }

        if *tokens.front().unwrap() == Token::And {
            tokens.pop_front();
            return true;
        }

        if !compile_compare(tokens, query) { return false; }
        query.push(Opcode::And);
    }

    return true;
}

fn compile_or(tokens: &mut VecDeque<Token>, query: &mut Vec<Opcode>) -> bool {
    if !compile_and(tokens, query) { return false; }
    loop {
        if tokens.is_empty() { break; }
        if *tokens.front().unwrap() == Token::Or {
            tokens.pop_front();
            return true;
        }

        if !compile_and(tokens, query) { return false; }
        query.push(Opcode::Or);
    }

    return true;
}

fn compile_expr(tokens: &mut VecDeque<Token>, query: &mut Vec<Opcode>) -> bool {
    if !compile_or(tokens, query) { return false; }
    return true;
}

fn compile(mut tokens: VecDeque<Token>) -> Option<Vec<Opcode>> {
    let mut stack: Vec<Opcode> = Vec::new();
    if compile_expr(&mut tokens, &mut stack) { return Some(stack); }
    return None;
}

pub fn parse_tql(args: String, debug: bool) -> Option<Vec<Opcode>>{
    let mut tokens: VecDeque<Token> = tokenize(args, debug);
    return compile(tokens);
}

pub fn task_matches_query(task: Task, stack: Vec<Opcode>, debug: bool) -> Option<Task> {
    if stack[0] == Opcode::Any { return Some(task); }
    
    todo!("Todo: task_matches_query");
}
