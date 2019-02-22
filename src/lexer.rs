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
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch as char {
            '=' => Token::new(TokenType::Assign, "=".to_string()),
            ';' => Token::new(TokenType::Semicolon, ";".to_string()),
            '(' => Token::new(TokenType::Lparen, "(".to_string()),
            ')' => Token::new(TokenType::Rparen, ")".to_string()),
            ',' => Token::new(TokenType::Comma, ",".to_string()),
            '+' => Token::new(TokenType::Plus, "+".to_string()),
            '{' => Token::new(TokenType::Lbrace, "{".to_string()),
            '}' => Token::new(TokenType::Rbrace, "}".to_string()),
            '\0' =>   Token::new(TokenType::EOF, tokens::EOF.to_string()),
            _ => panic!("Error: invalid token"),
            };
        

        self.read_char();
        tok
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
        String::from_utf8(self.input[position..self.position].to_vec()).expect("Unable to create String")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn next_token_test() {
        let input = "=+(){},;".as_bytes().to_vec();
        
        let tests: Vec<Token> = vec![
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

        let tests: Vec<Token> = vec![
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

    
    }
}