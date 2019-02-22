
//type TokenType = String;

pub struct LexerError {
    line: usize,
    token: TokenType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenType {
    Illegal(String),
    EOF,

    // Identifiers and literals:
    Ident(String),
    Int(i32),

    // Operators
    Assign,
    Plus,
    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keywords
    Function,
    Let,
}

// TODO: Change to just enum?
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub tokentype: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(tt: TokenType, lit: String) -> Token {
        Token {
            tokentype: tt,
            literal: lit,
        }
    }
}



pub mod tokens {
    pub const ILLEGAL: &str = "ILLEGAL";
    pub const EOF: &str = "EOF";

    // Identifiers + literals
    pub const IDENT: &str = "IDENT";
    pub const INT: &str = "INT";

    // Operators
    pub const ASSIGN: &str = "=";
    pub const PLUS: &str = "+";

    // Delimiters
    pub const COMMA: &str = ",";
    pub const SEMICOLON: &str = ";";


    pub const LPAREN: &str = "(";
    pub const RPAREN: &str = ")";
    pub const LBRACE: &str = "{";
    pub const RBRACE: &str = "}";

    // Keywords
    pub const FUNCTION: &str = "FUNCTION";
    pub const LET: &str = "LET";
}