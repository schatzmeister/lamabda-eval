use std::io::{stdin, stdout, Write};
use std::process::exit;

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

fn eval(input: &mut String) -> String {
    if input == ":quit" {
        exit(0);
    }
    input.clone()
}

fn main() {
    startup();
    
    loop {
        print!("> ");
        let mut input = String::new();
        read(&mut input);
        println!("{}", eval(&mut input))
    }
}
