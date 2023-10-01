#[derive(Debug, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Ident(String),
    String(String),
    GlobalVar(GlobalVar),
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum GlobalVar {
    Print,
}
