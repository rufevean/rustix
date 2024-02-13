struct Lexer {
    input: String,
    position: usize,
}
#[derive(Debug)]
struct Token {
    token_type: TokenType,
    literal: String,
}
impl Token {
    fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}
#[derive(Debug, PartialEq)]
enum TokenType {
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

    fn next_token(&mut self) {
        self.skip_whitespace();
        if self.position >= self.input.len() {
            Token::new(TokenType::TEof, "".to_string());
        }
        let mut current_char = self.input.chars().nth(self.position).unwrap();
        match current_char {
            '+' => {
                let current_token = Token::new(TokenType::TAdd, current_char.to_string());
                println!("{:?}", current_token);
                self.position += 1;
            }
            '-' => {
                let current_token = Token::new(TokenType::TSub, current_char.to_string());
                println!("{:?}", current_token);
                self.position += 1;
            }
            '*' => {
                let current_token = Token::new(TokenType::TMul, current_char.to_string());
                println!("{:?}", current_token);
                self.position += 1;
            }
            '/' => {
                let current_token = Token::new(TokenType::TDiv, current_char.to_string());
                println!("{:?}", current_token);
                self.position += 1;
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
                    println!("{:?}", current_token);
                } else {
                    let current_token = Token::new(TokenType::TEof, current_char.to_string());
                    println!("{:?}", current_token);

                    self.position += 1;
                }
            }
        };
    }
}

pub fn scan(contents: &String) {
    println!(" contains: {}", contents);
    let mut lexer = Lexer {
        input: contents.to_string(),
        position: 0,
    };
    loop {
        lexer.next_token();
    }
}
