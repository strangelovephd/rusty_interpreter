use crate::token::*;

pub struct Lexer {
    input: String,          //
    position: usize,        // current position in input (points to current char)
    read_position: usize,   // current reading position in input (after current char)
    ch: u8,               // current char under examination
}

impl Lexer {
    pub fn new(i: String) -> Lexer {
        Lexer {
            input: i,
            position: 0,
            read_position: 1,
            ch: 0,
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok = match self.ch as char {
            '=' => Token::new(tokens::ASSIGN.to_string(), self.ch.to_string()),
            ';' => Token::new(tokens::SEMICOLON.to_string(), self.ch.to_string()),
            '(' => Token::new(tokens::LPAREN.to_string(), self.ch.to_string()),
            ')' => Token::new(tokens::RPAREN.to_string(), self.ch.to_string()),
            ',' => Token::new(tokens::COMMA.to_string(), self.ch.to_string()),
            '+' => Token::new(tokens::PLUS.to_string(), self.ch.to_string()),
            '{' => Token::new(tokens::LBRACE.to_string(), self.ch.to_string()),
            '}' => Token::new(tokens::RBRACE.to_string(), self.ch.to_string()),
            //'\0' =>   Token::new("".to_string(), tokens::EOF.to_string()),
            _ => panic!("Error: invalid token"),
            };
        

        self.read_char();
        tok
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn next_token_test() {
        let input = "=+(){},;";
        let tests: Vec<Token> = vec![
            Token {tokentype: tokens::ASSIGN.to_string(), literal: "=".to_string()},
            Token {tokentype: tokens::PLUS.to_string(), literal: "+".to_string()},
            Token {tokentype: tokens::LPAREN.to_string(), literal: "(".to_string()},
            Token {tokentype: tokens::RPAREN.to_string(), literal: ")".to_string()},
            Token {tokentype: tokens::LBRACE.to_string(), literal: "{".to_string()},
            Token {tokentype: tokens::RBRACE.to_string(), literal: "}".to_string()},
            Token {tokentype: tokens::COMMA.to_string(), literal: ",".to_string()},
            Token {tokentype: tokens::SEMICOLON.to_string(), literal: ";".to_string()},
            Token {tokentype: tokens::EOF.to_string(), literal: "".to_string()},
        ];

        let mut l = Lexer::new(input.to_string());

        for t in tests.into_iter() {
            
            let tok = l.next_token();
            println!("lexer: {:?}", tok);
            println!("test: {:?}", t);
            assert_eq!(tok, t);
        }
    
    
    
    }
}