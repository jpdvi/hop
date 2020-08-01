use std::io;

pub fn start() {
    let mut input = String::new();
    println!("Welcome to hell...");
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            let mut l : super::lexer::Lexer = super::lexer::Lexer::new(&input);
            loop {
                let tok : super::token::Token = l.next_token();
                if tok._type == super::token::EOF{
                    break;
                }
                println!("Token [ type: {} | literal: {} ]", tok._type, tok.literal);
            }
        }
        Err(error) => println!("error: {}", error),
    }
}
