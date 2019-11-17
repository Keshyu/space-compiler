use std::fmt::{Display, Formatter, Result};


#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum TokenType {
    LET, TYPE,
    PLUS, MUL,
    EQ, NEQ,
    STRING, NUMBER, NAME, CHAR,
    BEGIN, END, NEWLINE,
    PRINT,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

