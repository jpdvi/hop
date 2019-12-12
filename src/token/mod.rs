#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub type TokenType = String;

pub struct Token {
    _type: TokenType,
    literal: String,
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
