use super::token;

pub trait Node {
    fn token_literal(&self) -> Option<&str>;
}

trait ExpressionNode: Node {
    fn expression_node();
}

pub trait Statement: Node {
    fn statement_node(&self);
    fn name(&self) -> Option<Identifier>;
    fn token(&self) -> token::Token;
}

#[derive(Clone, Debug)]
pub struct Expression {
    value: String
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl Identifier {
    pub fn new(token: &token::Token, value: &str) -> Self {
        Self {
            token: token.clone(),
            value: value.to_string(),
        }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> Option<&str> {
        return Some(self.token.literal.as_ref());
    }
}

#[derive(Clone, Debug)]
pub struct LetStatement {
    pub name: Option<Identifier>,
    pub token: token::Token,
    pub value: Option<Expression>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
    fn name(&self) -> Option<Identifier> {
        return self.name.clone()
    }
    fn token(&self) -> token::Token {
        return self.token.clone()
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> Option<&str> {
        return Some(self.token.literal.as_ref());
    }
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Self {
        Self { statements: vec![] }
    }
}

impl Node for Program {
    fn token_literal(&self) -> Option<&str> {
        if self.statements.len() as u32 > 0 {
            return self.statements[0].token_literal();
        }
        return None;
    }
}
