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
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok = match self.ch {
            '=' => Token::new(tokens::ASSIGN, self.ch),
            ';' => Token::new(tokens::SEMICOLON, self.ch),
            '(' => Token::new(tokens::LPAREN, self.ch),
            ')' => Token::new(tokens::RPAREN, self.ch),
            ',' => Token::new(tokens::COMMA, self.ch),
            '+' => Token::new(tokens::PLUS, self.ch),
            '{' => Token::new(tokens::LBRACE, self.ch),
            '}' => Token::new(tokens::RBRACE, self.ch),
            0 =>   Token {literal: "", tokentype: tokens::EOF,
            _ => panic!("Error: invalid token"),
            }
        }

        self.read_char();
        tok
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn NextTokentest() {
        let input = "=+(){},;";
        let tests: Vec<Token> = vec![
            Token {tokentype: ASSIGN, literal: "="},
            Token {tokentype: PLUS, literal: "+"},
            Token {tokentype: LPAREN, literal: "("},
            Token {tokentype: RPAREN, literal: ")"},
            Token {tokentype: LBRACE, literal: "{"},
            Token {tokentype: RBRACE, literal: "}"},
            Token {tokentype: COMMA, literal: ","},
            Token {tokentype: SEMICOLON, literal: ";"},
            Token {tokentype: EOF, literal: ""},
        ];

        let l = Lexer::new(input);

        for (i, t) in tests.into_iter().enumerate() {
            let tok = l.next_token();

            assert_eq!(tok, tests[i]);
        }
    
    
    
    }
}