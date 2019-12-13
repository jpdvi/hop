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
    input: &'a str,
    position: u32,
    read_position: u32,
    ch: char,
    keyword_map: KeywordMap,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: '0',
            keyword_map: KeywordMap::new(),
        };
        l.read_char();
        return l;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() as u32 {
            self.ch = ' ';
            self.position = self.read_position;
        } else {
            self.ch = self.input.chars().nth(self.read_position as usize).unwrap();
            self.position = self.read_position;
            self.read_position += 1;
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        let mut tok: token::Token;
        self.skip_whitespace();
        let mut can_read_char : bool = true;
        match self.ch {
            '=' => tok = token::Token::new(String::from(token::ASSIGN), self.ch),
            ';' => tok = token::Token::new(String::from(token::SEMICOLON), self.ch),
            '(' => tok = token::Token::new(String::from(token::LPAREN), self.ch),
            ')' => tok = token::Token::new(String::from(token::RPAREN), self.ch),
            ',' => tok = token::Token::new(String::from(token::COMMA), self.ch),
            '+' => tok = token::Token::new(String::from(token::PLUS), self.ch),
            '{' => tok = token::Token::new(String::from(token::LBRACE), self.ch),
            '}' => tok = token::Token::new(String::from(token::RBRACE), self.ch),
            ' ' =>  tok = token::Token::new(String::from(token::EOF), ' '),
            _ => {
                tok = token::Token::new(String::from(token::ILLEGAL), self.ch);
                if self.ch.is_alphabetic() || self.ch == '_' {
                    tok.literal = self.read_identifier();
                    tok._type = self.lookup_ident(&tok.literal);
                } else if self.ch.is_numeric() {
                    tok.literal = self.read_number();
                    tok._type = String::from(token::INT);
                    can_read_char = false;
                } else {
                    println!("Found Illegal {}", self.ch.to_string());
                    tok = token::Token::new(String::from(token::ILLEGAL), self.ch);
                }
            }
        }

        if can_read_char {
            self.read_char();
        }
        return tok;
    }

    pub fn skip_whitespace(&mut self) {
        if self.ch == ' ' || self.ch == '\n' || self.ch == '\t' || self.ch == '\r' {
            self.read_char();
        }
    }

    pub fn read_number(&mut self) -> String {
        let mut s: String = String::new();
        while self.ch.is_numeric() {
            s.push_str(&self.ch.to_string());
            self.read_char();
        }
        return s;
    }

    pub fn read_identifier(&mut self) -> String {
        let mut chs: String = String::new();
        while self.ch.is_alphabetic() == true {
            chs.push_str(&String::from(&self.ch.to_string()));
            self.read_char();
        }
        return chs;
    }

    pub fn lookup_ident(&mut self, ident: &str) -> token::TokenType {
        match self.keyword_map.get().get(ident) {
            Some(val) => {
                return String::from(val);
            }
            _ => return String::from(token::IDENT),
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
        let input: String = String::from("let five = 50;");
        let tests = vec![
            TestScheme {
                expected_type: String::from(super::token::LET),
                expected_literal: "let".to_string(),
            },
            TestScheme {
                expected_type: String::from(super::token::IDENT),
                expected_literal: "five".to_string(),
            },
            TestScheme {
                expected_type: String::from(super::token::ASSIGN),
                expected_literal: '='.to_string(),
            },
            TestScheme {
                expected_type: String::from(super::token::INT),
                expected_literal: "50".to_string(),
            },
            TestScheme {
                expected_type: String::from(super::token::SEMICOLON),
                expected_literal: ';'.to_string(),
            },
            TestScheme {
                expected_type: String::from(super::token::EOF),
                expected_literal: ' '.to_string(),
            },
        ];
        let mut l: super::Lexer = super::Lexer::new(&input);
        for i in tests.iter() {
            let tok: super::token::Token = l.next_token();
            assert_eq!(tok._type, i.expected_type);
            assert_eq!(tok.literal, i.expected_literal);
        }
    }
}
