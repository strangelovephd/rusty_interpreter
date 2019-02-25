use crate::token::*;



pub struct Lexer {
    input: Vec<u8>,          
    position: usize,        // current position in input (points to current char)
    read_position: usize,   // current reading position in input (after current char)
    ch: char,               // current char under examination
}

impl Lexer {
    pub fn new(i: Vec<u8>) -> Lexer {
        let mut l = Lexer {
            input: i,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
        println!("Read: {}", self.ch);
    }

/*     pub fn _next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => Token::new(TokenType::Assign, "=".to_string()),
            ';' => Token::new(TokenType::Semicolon, ";".to_string()),
            '(' => Token::new(TokenType::Lparen, "(".to_string()),
            ')' => Token::new(TokenType::Rparen, ")".to_string()),
            ',' => Token::new(TokenType::Comma, ",".to_string()),
            '+' => Token::new(TokenType::Plus, "+".to_string()),
            '{' => Token::new(TokenType::Lbrace, "{".to_string()),
            '}' => Token::new(TokenType::Rbrace, "}".to_string()),
            '\0' => Token::new(TokenType::EOF, tokens::EOF.to_string()),
            _ => {
                if Lexer::is_char(&self.ch) {
                    let literal = self.read_identifier();
                    let kw = match look_up_keyword(&literal) {
                        Some(kw) => {
                            match kw {
                                Keyword::Function => TokenType::Function,
                                Keyword::Let => TokenType::Let,
                            }
                        },
                        None => TokenType::IllegalIdent(literal.clone()),
                    };

                    Token::new(kw, literal)
                } else if Lexer::is_digit(&self.ch) {
                    Token::new(TokenType)
                } else {
                    Token::new(TokenType::Illegal(String::new()), self.ch.to_string())
                }
            }
            };
        

        self.read_char();
        tok
    } */

    pub fn next_token(&mut self) -> TokenType {
        self.skip_whitespace();

        let tt = match look_up_token(&self.ch.to_string()) {
            Some(e) => e,
            None => {
                if Lexer::is_char(&self.ch) {
                    let literal = self.read_identifier();
                    let kw = match look_up_keyword(&literal) {
                        Some(kw) => {
                            match kw {
                                Keyword::Function => TokenType::Function,
                                Keyword::Let => TokenType::Let,
                            }
                        },
                        None => TokenType::Ident(literal),
                    };
                    kw
                } else if Lexer::is_digit(&self.ch) {
                    TokenType::Int(self.read_number())
                } else {
                    TokenType::Illegal(self.ch.to_string())
                }
            }
        };
        self.read_char();
        tt
    }

    fn is_letter(ch: &u8) -> bool {
      //'a'                   'z'      'A'                   'Z'            '_'
        97u8 <= *ch && *ch <= 122u8 || 65u8 <= *ch && *ch <= 90u8 || *ch == 95u8
    }

    fn is_char(ch: &char) -> bool {
        'a' <= *ch && *ch <= 'z' || 'A' <= *ch && *ch <= 'Z' || *ch == '_'
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while Self::is_char(&self.ch) {
            self.read_char(); 
        }
        println!("Identifier read: {} to {}", position, self.position);
        String::from_utf8(self.input[position..self.position]
            .to_vec())
            .expect("Unable to create String from Identifier")
    }

    fn skip_whitespace(&mut self) {
        println!("nom whitespace");
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        } 
    }

    fn read_number(&mut self) -> i32 {
        let position = self.position;
        while Lexer::is_digit(&self.ch) {
            self.read_char();
        }

        println!("Making int\nposition: {} to {}\n{}", position, self.position, String::from_utf8(self.input[position..self.position].to_vec()).unwrap());
        
        String::from_utf8(self.input[position..self.position]
            .to_vec())
            .expect("Unable to create String from Digit")
            .parse::<i32>()
            .expect("Unable to parse int")
    }

    fn is_digit(byte: &char) -> bool {
        '0' <= *byte && *byte <= '9'
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn next_token_test() {
        let input = "=+(){},;".as_bytes().to_vec();
        
        let _tests: Vec<Token> = vec![
            Token {tokentype: TokenType::Assign, literal: "=".to_string()},
            Token {tokentype: TokenType::Plus, literal: "+".to_string()},
            Token {tokentype: TokenType::Lparen, literal: "(".to_string()},
            Token {tokentype: TokenType::Rparen, literal: ")".to_string()},
            Token {tokentype: TokenType::Lbrace, literal: "{".to_string()},
            Token {tokentype: TokenType::Rbrace, literal: "}".to_string()},
            Token {tokentype: TokenType::Comma, literal: ",".to_string()},
            Token {tokentype: TokenType::Semicolon, literal: ";".to_string()},
            Token {tokentype: TokenType::EOF, literal: "EOF".to_string()},
        ];

        let tests: Vec<TokenType> = vec![
            TokenType::Assign,
            TokenType::Plus,
            TokenType::Lparen,
            TokenType::Rparen,
            TokenType::Lbrace,
            TokenType::Rbrace,
            TokenType::Comma,
            TokenType::Semicolon,
            TokenType::EOF,
        ];

        let mut l = Lexer::new(input);

        for t in tests.into_iter() {
            let tok = l.next_token();
            println!("lexer: {:?}", tok);
            println!("test: {:?}", t);
            assert_eq!(tok, t);
        }
    }

    #[test]
    fn next_token_test_2() {
        let input = 
        "let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
            x + y;
        };
        
        let result = add(five, ten);".as_bytes().to_vec();

        let _tests: Vec<Token> = vec![
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident("five".to_string()), "five".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Int(5), "5".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident("ten".to_string()), "ten".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Int(10), "10".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident("add".to_string()), "add".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Function, "fn".to_string()),
            Token::new(TokenType::Lparen, "(".to_string()),
            Token::new(TokenType::Ident("x".to_string()), "x".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Ident("y".to_string()), "y".to_string()),
            Token::new(TokenType::Rparen, ")".to_string()),
            Token::new(TokenType::Lbrace, "{".to_string()),
            Token::new(TokenType::Ident("x".to_string()), "x".to_string()),
            Token::new(TokenType::Plus, "+".to_string()),
            Token::new(TokenType::Ident("y".to_string()), "y".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Rbrace, "}".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident("result".to_string()), "result".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Ident("add".to_string()), "add".to_string()),
            Token::new(TokenType::Lparen, "(".to_string()),
            Token::new(TokenType::Ident("five".to_string()), "five".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Ident("ten".to_string()), "ten".to_string()),
            Token::new(TokenType::Rparen, ")".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::EOF, "EOF".to_string()),
        ];

        let tests: Vec<TokenType> = vec![
            TokenType::Let,
            TokenType::Ident("five".to_string()),
            TokenType::Assign,
            TokenType::Int(5),
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident("ten".to_string()),
            TokenType::Assign,
            TokenType::Int(10),
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident("add".to_string()),
            TokenType::Assign,
            TokenType::Function,
            TokenType::Lparen,
            TokenType::Ident("x".to_string()),
            TokenType::Comma,
            TokenType::Ident("y".to_string()),
            TokenType::Rparen,
            TokenType::Lbrace,
            TokenType::Ident("x".to_string()),
            TokenType::Plus,
            TokenType::Ident("y".to_string()),
            TokenType::Semicolon,
            TokenType::Rbrace,
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident("result".to_string()),
            TokenType::Assign,
            TokenType::Ident("add".to_string()),
            TokenType::Lparen, 
            TokenType::Ident("five".to_string()),
            TokenType::Comma, 
            TokenType::Ident("ten".to_string()),
            TokenType::Rparen, 
            TokenType::Semicolon,
            TokenType::EOF,
        ];
        
        let mut l = Lexer::new(input);

        for t in tests.into_iter() {
            let tok = l.next_token();
            println!("lexer: {:?}", tok);
            println!("test: {:?}\n", t);
            assert_eq!(tok, t);
        }
    
    }
}