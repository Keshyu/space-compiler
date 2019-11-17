use super::token_type::TokenType;
use super::token_type::TokenType::*;


#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Name(String),
    // TODO: Add number types for all sizes
    // TODO: Divide Number into Integer and Float
    Number(i32),
    String(String),
    Char(char),
    Symbol(TokenType),
}

impl Token {
    pub fn r#type(&self) -> TokenType {
        match self {
            Token::Name(_) => NAME,
            Token::Number(_) => NUMBER,
            Token::String(_) => STRING,
            Token::Char(_) => CHAR,
            Token::Symbol(tt) => tt.clone(),
        }
    }
}
