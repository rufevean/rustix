
# Lexical Analysis or Scanning for simple operators.

Its just a simple process of breaking down the code in tokens , but to be cool, lets call it lexer ( which it is ). Lets start our language with four simple operator and a int literal.

```
    + , - , / , * and a number 
```

### Creating the test file and reading contents from it 

We create a directory in our repo named ``` Test ```, we keep our input files there.

``` rust 
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let name = &args[1];
    let contents = fs::read_to_string(name);
    match contents {
        Ok(contents) => println!("Contents : {}", contents),
        Err(e) => println!("Error reading {}: {}", name, e),
    }

}
```
Using std::env and std::fs , we take the file input and read the contents of that file , We use a match statement to handle errors while reading the file contents 

input :
``` bash
cargo run test/main.exs
```
output :
``` bash
contents :   1/2+2*3
```
## Creating a lexer 

We create a new directory named lexer and file named lexer.rs in the src directory . We add a file scan.rs in lexer directory .

in lexer.rs :
``` rust 
pub mod scan
```
in scan.rs :

We create a lexer and token structs and a tokentype enum. Lexer is for having the input and its position, token is for having the token type and its value and tokentype is tokentype :).

```  rust
struct Lexer {
    input: String,
    position: usize,
}
#[derive(Debug)]
struct Token {
    token_type: TokenType,
    literal: String,
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
```

We now loop through each character and parse them through a match statement printing the token . 
Lets first handle whitespaces tho,they suck ( sorry python ).

```rust
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
   }
```

Now, we loop the input and parse them into another method of lexer .
``` rust 
// looping through input 
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
```

Parsing input through and printing the output :

``` rust 
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
```
We parse through every single character and match them creating a new token which we can then print, to handle int litral, we first check if the next character is also an int and push till its not . 

output of the code :
``` bash

Token { token_type: TInt, literal: "1" }
Token { token_type: TDiv, literal: "/" }
Token { token_type: TInt, literal: "2" }
Token { token_type: TAdd, literal: "+" }
Token { token_type: TInt, literal: "2" }
Token { token_type: TMul, literal: "*" }
Token { token_type: TInt, literal: "3" }
```
Lets deal with the parser next . 













