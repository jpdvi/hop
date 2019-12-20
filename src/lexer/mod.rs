use super::token;
use std::collections::HashMap;

pub struct KeywordMap {
    data: HashMap<String, String>,
}

impl KeywordMap {
    pub fn new() -> KeywordMap {
        let map: HashMap<String, String> = [
            (String::from("let"), String::from(token::LET)),
            (String::from("fn"), String::from(token::FUNCTION)),
            (String::from("true"), String::from(token::TRUE)),
            (String::from("false"), String::from(token::FALSE)),
            (String::from("if"), String::from(token::IF)),
            (String::from("else"), String::from(token::ELSE)),
            (String::from("return"), String::from(token::RETURN)),
        ]
        .iter()
        .cloned()
        .collect();
        return KeywordMap { data: map };
    }

    pub fn get(&self) -> &HashMap<String, String> {
        return &self.data;
    }
}

// Lexer
pub struct Lexer<'a> {
    pub input: &'a str,
    pub position: u32,
    pub read_position: u32,
    pub ch: Option<char>,
    pub keyword_map: KeywordMap,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: None,
            keyword_map: KeywordMap::new(),
        };
        l.read_char();
        return l;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() as u32 {
            self.ch = None;
        } else {
            self.ch = Some(self.input.chars().nth(self.read_position as usize).unwrap());
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();
        let tok: token::Token = match self.ch {
            None => return token::Token::new(token::EOF, None),
            Some(';') => token::Token::new(token::SEMICOLON, self.ch),
            Some('(') => token::Token::new(token::LPAREN, self.ch),
            Some(')') => token::Token::new(token::RPAREN, self.ch),
            Some(',') => token::Token::new(token::COMMA, self.ch),
            Some('+') => token::Token::new(token::PLUS, self.ch),
            Some('{') => token::Token::new(token::LBRACE, self.ch),
            Some('}') => token::Token::new(token::RBRACE, self.ch),
            Some('-') => token::Token::new(token::MINUS, self.ch),
            Some('*') => token::Token::new(token::ASTRISK, self.ch),
            Some('/') => token::Token::new(token::SLASH, self.ch),
            Some('<') => token::Token::new(token::LT, self.ch),
            Some('>') => token::Token::new(token::GT, self.ch),
            Some('=') => {
                if self.peek_char() != None  && self.peek_char().unwrap() == '=' {
                    //  If next character is also = assume token is == 
                    //  then read chars until == is written to literal
                    let mut t : token::Token = token::Token::new(token::EQUAL, None); 
                    t.literal = self.ch.unwrap().to_string() + &self.peek_char().unwrap().to_string();
                    self.read_char();
                    t
                } else { 
                    let t = token::Token::new(token::ASSIGN, self.ch);
                    self.read_char();
                    t
                }
            },
            Some('!') => {
                if self.peek_char() != None && self.peek_char().unwrap() == '=' {
                    let mut t : token::Token = token::Token::new(token::NOT_EQUAL, None);
                    t.literal = self.ch.unwrap().to_string() + &self.peek_char().unwrap().to_string();
                    self.read_char();
                    t
                } else {
                    let t  = token::Token::new(token::BANG, self.ch);
                    t                
                }
            },
            y if y.unwrap().is_alphabetic() || y.unwrap() == '_' => {
                let l: String = self.read_identifier();
                let _type = self.lookup_ident(&l);
                let mut t = token::Token::new(_type, None);
                t.literal = l;
                return t;
            }
            y if y.unwrap().is_numeric() => {
                let n = self.read_number();
                let mut t = token::Token::new(token::INT, None);
                t.literal = n;
                return t;
            }
            _ => token::Token::new(token::ILLEGAL, None),
        };
        self.read_char();
        return tok;
    }

    pub fn skip_whitespace(&mut self) {
        if self.read_position >= self.input.len() as u32 {
            self.ch = None;
            return
        }

        while self.ch.unwrap() == ' '
            || self.ch.unwrap() == '\n'
            || self.ch.unwrap() == '\t'
            || self.ch.unwrap() == '\r'
        {
            self.read_char();
        }
    }

    pub fn read_number(&mut self) -> String {
        let mut s: String = String::new();
        while self.ch.unwrap().is_numeric() {
            s.push_str(&self.ch.unwrap().to_string());
            self.read_char();
        }
        return s;
    }

    pub fn read_identifier(&mut self) -> String {
        let mut chs: String = String::new();
        while self.ch.unwrap().is_alphabetic() == true {
            chs.push_str(&String::from(&self.ch.unwrap().to_string()));
            self.read_char();
        }
        return chs;
    }

    pub fn lookup_ident(&mut self, ident: &str) -> token::TokenType {
        match self.keyword_map.get().get(ident) {
            Some(val) => {
                return &val;
            }
            _ => return token::IDENT
        }
    }
    
    pub fn peek_char(&mut self) -> Option<char> {
        if self.read_position >= self.input.len() as u32 { 
            return None;  
        } else {
            return Some(self.input.chars().nth(self.read_position as usize).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_next_token() {
        struct TestScheme {
            expected_type: String,
            expected_literal: String,
        }
        
        impl TestScheme {
            fn from_string(exp_literal: &str, exp_type: &str) -> TestScheme {
                TestScheme {
                    expected_type : String::from(exp_type),
                    expected_literal : exp_literal.to_string()
                }
            }

            fn from_char(exp_literal: char, exp_type: &str) -> TestScheme {
                TestScheme {
                    expected_type : String::from(exp_type),
                    expected_literal : exp_literal.to_string() 
                }
            }
        }

        let input: String = String::from(
            "let five = 5; \n\t\r 
             let ten = 10;
             let add = fn(x, y) {
                x + y;
             };
             let result = add(five, ten);
             !-/*5;
             5 < 10 > 5;
             if (5 < 10) {
                return true;
             } else {
                return false;
             }
             10 == 10;
             10 != 9;",
        );
        let tests = vec![
            TestScheme::from_string("let", super::token::LET),
            TestScheme::from_string("five", super::token::IDENT),
            TestScheme::from_char('=', super::token::ASSIGN),
            TestScheme::from_string("5", super::token::INT),
            TestScheme::from_char(';', super::token::SEMICOLON),
            TestScheme::from_string("let", super::token::LET),
            TestScheme::from_string("ten", super::token::IDENT),
            TestScheme::from_char('=', super::token::ASSIGN),
            TestScheme::from_string("10", super::token::INT),
            TestScheme::from_char(';', super::token::SEMICOLON),
            TestScheme::from_string("let", super::token::LET),
            TestScheme::from_string("add", super::token::IDENT),
            TestScheme::from_char('=', super::token::ASSIGN),
            TestScheme::from_string("fn", super::token::FUNCTION),
            TestScheme::from_char('(', super::token::LPAREN),
            TestScheme::from_string("x", super::token::IDENT),
            TestScheme::from_char(',', super::token::COMMA),
            TestScheme::from_string("y", super::token::IDENT),
            TestScheme::from_char(')', super::token::RPAREN),
            TestScheme::from_char('{', super::token::LBRACE),
            TestScheme::from_string("x", super::token::IDENT),
            TestScheme::from_char('+', super::token::PLUS),
            TestScheme::from_string("y", super::token::IDENT),
            TestScheme::from_char(';', super::token::SEMICOLON),
            TestScheme::from_char('}', super::token::RBRACE),
            TestScheme::from_char(';', super::token::SEMICOLON),
            TestScheme::from_string("let", super::token::LET),
            TestScheme::from_string("result", super::token::IDENT),
            TestScheme::from_char('=', super::token::ASSIGN),
            TestScheme::from_string("add", super::token::IDENT),
            TestScheme::from_char('(', super::token::LPAREN),
            TestScheme::from_string("five", super::token::IDENT),
            TestScheme::from_char(',', super::token::COMMA),
            TestScheme::from_string("ten", super::token::IDENT),
            TestScheme::from_char(')', super::token::RPAREN),
            TestScheme::from_char(';', super::token::SEMICOLON),
            TestScheme::from_char('!', super::token::BANG),
            TestScheme::from_char('-', super::token::MINUS),
            TestScheme::from_char('/', super::token::SLASH),
            TestScheme::from_char('*', super::token::ASTRISK),
            TestScheme::from_char('5', super::token::INT),
            TestScheme::from_char(';', super::token::SEMICOLON),
            TestScheme::from_char('5', super::token::INT),
            TestScheme::from_char('<', super::token::LT),
            TestScheme::from_string("10", super::token::INT),
            TestScheme::from_char('>', super::token::GT),
            TestScheme::from_char('5', super::token::INT),
            TestScheme::from_char(';', super::token::SEMICOLON),
            TestScheme::from_string("if", super::token::IF),
            TestScheme::from_char('(', super::token::LPAREN),
            TestScheme::from_char('5', super::token::INT),
            TestScheme::from_char('<', super::token::LT),
            TestScheme::from_string("10", super::token::INT),
            TestScheme::from_char(')', super::token::RPAREN),
            TestScheme::from_char('{', super::token::LBRACE),
            TestScheme::from_string("return", super::token::RETURN),
            TestScheme::from_string("true", super::token::TRUE),
            TestScheme::from_char(';', super::token::SEMICOLON),
            TestScheme::from_char('}', super::token::RBRACE),
            TestScheme::from_string("else", super::token::ELSE),
            TestScheme::from_char('{', super::token::LBRACE),
            TestScheme::from_string("return", super::token::RETURN),
            TestScheme::from_string("false", super::token::FALSE),
            TestScheme::from_char(';', super::token::SEMICOLON),
            TestScheme::from_char('}', super::token::RBRACE),
            TestScheme::from_string("10", super::token::INT),
            TestScheme::from_string("==", super::token::EQUAL),
            TestScheme::from_string("10", super::token::INT),
            TestScheme::from_char(';', super::token::SEMICOLON),
            TestScheme::from_string("10", super::token::INT),
            TestScheme::from_string("!=", super::token::NOT_EQUAL),
            TestScheme::from_string("9", super::token::INT),
            TestScheme::from_char(';', super::token::SEMICOLON),
        ];

        let mut l: super::Lexer = super::Lexer::new(&input);
        for i in tests.iter() {
            let tok: super::token::Token = l.next_token();
            println!("{} {}", tok._type, i.expected_type);
            println!("{} {}", tok.literal, i.expected_literal);
            assert_eq!(tok._type, i.expected_type);
            assert_eq!(tok.literal, i.expected_literal);
        }
    }
}
