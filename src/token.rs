use lazy_static::lazy_static;

use std::collections::HashMap;

//type TokenType = String;

pub struct LexerError {
    line: usize,
    token: TokenType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenType {
    Illegal(String),
    IllegalIdent(String),
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

lazy_static!{
    static ref TOKENS: HashMap<&'static str, TokenType> = {
        let mut tt = HashMap::new();
        
        tt.insert("\u{0}", TokenType::EOF);
        
        // Operators
        tt.insert("=", TokenType::Assign);
        tt.insert("+", TokenType::Plus);
        
        // Delimiters
        tt.insert(",", TokenType::Comma);
        tt.insert(";", TokenType::Semicolon);
        tt.insert("(", TokenType::Lparen);
        tt.insert(")", TokenType::Rparen);
        tt.insert("{", TokenType::Lbrace);
        tt.insert("}", TokenType::Rbrace);
        
        // Keywords
        tt.insert("fn", TokenType::Function);
        tt.insert("let", TokenType::Let);
        tt
    };
}

pub fn look_up_token(tok: &str) -> Option<TokenType> {
    match TOKENS.get(tok) {
        Some(e) => Some(e.clone()),
        None => None,
    }
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Keyword {
    Function,
    Let,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Keyword> = {
        let mut kw = HashMap::new();
        kw.insert("fn", Keyword::Function);
        kw.insert("let", Keyword::Let);
        kw
    };
}

pub fn look_up_keyword(kw: &str) -> Option<Keyword> {
    match KEYWORDS.get(kw) {
        Some(e) => Some(e.clone()),
        None => None,
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn look_up_test() {
        let tests: Vec<&str> = vec![
            "=",
            "+",
            ",",
            ";",
            "(",
            ")",
            "{",
            "}",
            "fn", 
            "let",
        ];
        assert_eq!(TokenType::Assign, look_up_token("=").unwrap());
        assert_eq!(TokenType::Semicolon, look_up_token(";").unwrap());
        assert_eq!(TokenType::Let, look_up_token("let").unwrap());
        assert_eq!(TokenType::Function, look_up_token("fn").unwrap());

    }
}