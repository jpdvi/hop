pub type TokenType = String;

pub struct Token {
    pub _type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: char) -> Token {
        Token {
            _type: token_type,
            literal: ch.to_string(),
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
