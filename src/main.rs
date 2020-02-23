use std::io::{stdin, stdout, Write};
use std::process::exit;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Token {
    Lambda,
    Param(String),
    Dot,
    Body(String),
    Command(String),
}

fn startup() {
    println!("Welcome to Λeval v{}", env!("CARGO_PKG_VERSION"));
}

fn read(mut input: &mut String) {
    let _ = stdout().flush();
    match stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(error) => println!("Couldn’t handle input: {}", error),
    }
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
}

fn tokenize(s: String) -> Vec<Token> {
    let mut toks = Vec::new();
    let mut sit = s.chars().peekable();
    
    loop {
        let ch = sit.next();
        if ch == None {
            break;
        }
        let ch = ch.unwrap();
        
        if ch == ':' {
            return vec![Token::Command(s[1..].to_string())];
        }
        
        if ch == 'λ' || ch == '\\' {
            toks.push(Token::Lambda)
        } else if ch == '.' {
            toks.push(Token::Dot)
        } else {
            let mut cont = String::new();
            cont.push(ch);
            while let Some(x) = sit.peek() {
                match x {
                    '.' | 'λ' | '\\' => break,
                    _ => {
                        cont.push(*x);
                        sit.next();
                    },
                }
            }
            match sit.next() {
                None => (),
                Some('.') => toks.push(Token::Param(cont)),
                _ => toks.push(Token::Body(cont)),
            }
        }
    }
    toks
}

fn eval(input: Vec<Token>) -> String {
    if input.is_empty() {
        return String::new()
    } else if input[0] == Token::Command("quit".to_string()) {
        exit(0);
    }
    format!("{:?}", input)
}

fn main() {
    startup();
    
    loop {
        print!("> ");
        let mut input = String::new();
        read(&mut input);
        println!("{}", eval(tokenize(input)));
    }
}
