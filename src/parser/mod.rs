use super::ast;
use super::lexer;
use super::token;

pub struct Parser {
    l: lexer::Lexer,
    current_token: Option<token::Token>,
    peek_token: Option<token::Token>,
    errors: Vec<String>,
}

impl Parser {
    fn new(lexer: lexer::Lexer) -> Self {
        let mut parser = Parser {
            l: lexer,
            current_token: None,
            peek_token: None,
            errors: vec![],
        };
        parser.next_token();
        parser.next_token();
        return parser;
    }

    pub fn get_errors(&self) -> Vec<String> {
        return self.errors.clone()
    }

    fn peek_error(&mut self, token_type: token::TokenType) {
        self.errors.push(
            format!("expected next token to be of type {} instead received {}", 
                self.peek_token.clone().unwrap()._type, token_type));
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.l.next_token().clone());
    }

    fn current_token_is(&self, tok: token::TokenType) -> bool {
        if let Some(t) = self.current_token.as_ref() {
            return t._type == tok;
        }
        false
    }

    fn peek_token_is(&self, tok: token::TokenType) -> bool {
        if let Some(t) = self.peek_token.as_ref() {
            return t._type == tok;
        }
        false
    }

    fn expect_peek(&mut self, tok: token::TokenType) -> bool {
        if self.peek_token_is(tok) {
            self.next_token();
            return true;
        }
        self.peek_error(tok);
        false
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        if let Some(_current_token)  = self.current_token.clone() {
            let mut statement = ast::LetStatement { 
                name: None, 
                token : _current_token.clone(),
                value: None
            };

            if !self.expect_peek(token::IDENT) {
                return None
            }
            println!("{:?}", &_current_token.literal);
            statement.name = Some(ast::Identifier::new(
                    &self.current_token.clone().unwrap(),
                    &self.current_token.clone().unwrap().literal));

            if !self.current_token_is(token::SEMICOLON) {
                self.next_token();
            }
            self.next_token();
            return Some(Box::new(statement))
        }
        None
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        if let Some(_current_token) = &self.current_token {
            return match _current_token._type.as_ref() {
                token::LET => self.parse_let_statement(),
                _ => None,
            };
        }
        None
    }

    fn parse_program(&mut self) -> Result<ast::Program, &str> {
        let mut program = ast::Program::new();
        while self.current_token.clone().unwrap()._type != token::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        Ok(program)
    }
}

#[cfg(test)]
mod test {
    use super::lexer;
    use super::Parser;
    use super::ast;
    use crate::ast::Node;

    struct Test {
        expected_identifier: String,
    }

    #[test]
    #[allow(dead_code)]
    fn test_let_statement() {
        let input: &str = "let x = 5;let y = 10;let foobar = 838383;";
        let lex = lexer::Lexer::new(input);
        let mut p = Parser::new(lex);
        let program = p.parse_program();
        assert!(!program.is_err(), "An error occured in program");
        if let Ok(program) = program {
            if program.statements.len() != 3 {
                assert!(false, "program does not contain 3 statements");
            }
            let tests = vec![
                Test {
                    expected_identifier: "x".to_string(),
                },
                Test {
                    expected_identifier: "y".to_string(),
                },
                Test {
                    expected_identifier: "foobar".to_string(),
                },
            ];
            for i in 0..tests.len() {
                _test_let_statement(&*program.statements[i], &tests[i]);
            }
        }     
    }

    fn _test_let_statement(s: &dyn ast::Statement, _name: &Test) {
        assert_eq!(s.token_literal().unwrap(), "let");
        assert_eq!(s.name().clone().unwrap().value, _name.expected_identifier);
    }
}
