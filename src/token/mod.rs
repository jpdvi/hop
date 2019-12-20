pub type TokenType<'a> = &'a str;

pub struct Token<'a> {
    pub _type: TokenType<'a>,
    pub literal: String,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, ch: Option<char>) -> Token {
        let mut l = ' '.to_string();
        if ch != None {
            l   = ch.unwrap().to_string();
        }
        Token {
            _type: token_type,
            literal: l,
        }
    }
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";
//Identifiers and Literals
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";
// Operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTRISK: &str = "*";
pub const SLASH: &str = "/";
pub const LT: &str = "<";
pub const GT: &str = ">";
// Delimeters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";
// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub const EQUAL: &str = "==";
pub const NOT_EQUAL: &str = "!=";
// Control Flow
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";
