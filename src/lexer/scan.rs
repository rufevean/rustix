pub struct Lexer {
    input: String,
    position: usize,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
impl Token {
    fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    TInt,
    TSub,
    TAdd,
    TEof,
    TMul,
    TDiv,
}

impl Lexer {
    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self
                .input
                .chars()
                .nth(self.position)
                .unwrap()
                .is_whitespace()
        {
            self.position += 1;
        }
    }
    fn is_digit(&mut self, c: char) -> bool {
        c.is_digit(10)
    }

    fn handle_escape_char(&mut self) {
        if self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            if current_char == '\\' {
                self.position += 1;
            } else if current_char == '\n' {
                self.position += 1;
            } else if current_char == '\t' {
                self.position += 1;
            }
        }
    }
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            position: 0,
        }
    }
    fn current_char(&self) -> char {
        if self.position >= self.input.len() {
            '\0'
        } else {
            Some(self.input.as_bytes()[self.position]).unwrap() as char
        }
    }
    pub fn scan(&mut self) -> Token {
        self.skip_whitespace();
        self.handle_escape_char();
        if self.position >= self.input.len() {
            Token::new(TokenType::TEof, "".to_string());
        }
        let mut current_char = self.current_char();
        match current_char {
            '+' => {
                let current_token = Token::new(TokenType::TAdd, current_char.to_string());
                self.position += 1;
                current_token 

            }
            '-' => {
                let current_token = Token::new(TokenType::TSub, current_char.to_string());
                self.position += 1;
                current_token
            }
            '*' => {
                let current_token = Token::new(TokenType::TMul, current_char.to_string());
                self.position += 1;
                current_token
            }
            '/' => {
                let current_token = Token::new(TokenType::TDiv, current_char.to_string());
                self.position += 1;
                current_token
            }

            _ => {
                if self.is_digit(current_char) {
                    let mut number = String::new();
                    while self.is_digit(current_char) {
                        number.push(current_char);
                        self.position += 1;
                        current_char = self.input.chars().nth(self.position).unwrap();
                    }
                    let current_token = Token::new(TokenType::TInt, number);
                    current_token
                } else {
                    let current_token = Token::new(TokenType::TEof, current_char.to_string());

                    self.position += 1;
                    current_token
                }
            }
        }
    }
}

