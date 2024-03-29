# Basic Parsing

Before parsing, lets just minify the input code so that we dont have deal with whitespaces . its easy 
``` rust 
pub fn minify(contents : &str) -> String{
    println!("Minifying: {}", contents);
    let mut minified = String::new();
    for c in contents.chars(){
        if c.is_whitespace() {
            continue;
        }
        minified.push(c);
    }
    minified
}
```
In a compiler, a parser is responsible for analyzing the syntactic structure of the source code and transforming it into a format that can be further processed by the compiler. The parser takes the stream of tokens generated by the lexical analyzer (tokenizer) and constructs a parse tree or abstract syntax tree (AST) representing the hierarchical structure of the code according to the grammar rules of the programming language.

Lets also interpret out AST we get from the parser .

``` elixir 
1234 + -56 * / - - 8 + * 2
```
We are going to use this input and first tokenize stuff and the build a AST .

In defs.rs , lets define the node type and the tree
``` rust

#[derive(Debug)]
pub struct ASTnode {
    pub op: Option<ASTtype>,
    pub left: Option<Box<ASTnode>>,
    pub right: Option<Box<ASTnode>>,
    pub value: i32,
}
// AST node types
#[derive(Debug)]
pub enum ASTtype {
    Aadd,
    Asub,
    Amul,
    Adiv,
    Aint,
    Aunknown,
}
```
Our AST will be simple for now as we are only playing with simple expressions. Lets now define the functions we are going to use new leaves and nodes.
in parse.rs
``` rust 

pub fn new_ast_node(
    op: Option<ASTtype>,
    left: Option<Box<ASTnode>>,
    right: Option<Box<ASTnode>>,
    value: i32,
) -> ASTnode {
    ASTnode {
        op,
        left: Some(left).unwrap_or_else(|| None),
        right: Some(right).unwrap_or_else(|| None),
        value,
    }
}

pub fn new_ast_leaf(value: i32) -> ASTnode {
    new_ast_node(Some(ASTtype::Aint), None, None, value)
}

``` 
Until now, we have created a structure for AST and added functions to add data into it. Now lets try to use our tokens to build one AST.

``` rust
pub fn parse(input: &str) {
    println!("Parsing: {}", input);
    let mut lexer = Lexer::new(input.to_string());
    let token = lexer.scan();
    let ast = binexpr(&mut lexer, token);
    }
   ```
   in function binexpr() , we check if the token is a operator, if yes, we then assign a ASTtype to it, and create a new node for the operator and then we keep on scanning . 
  ``` rust 
  pub fn binexpr(lexer: &mut Lexer, token: Token) -> ASTnode {
    let mut left = primary(&token);
    let mut token = lexer.scan();
    if token.token_type == TEof {
        return left;
    }
    while token.token_type == TAdd
        || token.token_type == TSub
        || token.token_type == TMul
        || token.token_type == TDiv
    {
        let op = arithop(&token);
        let right = primary(&lexer.scan());
        left = new_ast_node(Some(op) , Some(Box::new(left)), Some(Box::new(right)), 0);
        token = lexer.scan();
    }
    left
}
```
in arithop() , its straight forward, simple match statement to assign AST type to the operators. 
``` rust 

pub fn arithop(token: &Token) -> ASTtype {
    match &token.token_type {
        TAdd => ASTtype::Aadd,
        TSub => ASTtype::Asub,
        TMul => ASTtype::Amul,
        TDiv => ASTtype::Adiv,
        _ => {
            println!("Unknown token: {:?}", token.token_type);
            std::process::exit(1);
        }
    }
}
``` 

in primary(), we check if the token is a number so that we can create a leaf , the error statements work if we by mistake add two operators without an int literal between them, like ``` 1+/2```. 
``` rust
pub fn primary(token: &Token) -> ASTnode {
    match &token.token_type {
        TInt => {
            let node = new_ast_leaf(token.literal.parse::<i32>().unwrap());
            node
        }
        _ => {
            println!("Unknown token: {:?}", token.token_type);
            std::process::exit(1);
        }
    }
}
``` 
Thats it, we created out first AST . 
the output looks something like this 
```
Minifying: 1234 + -56 * / - - 8 + * 2

Parsing: 1234+-56*/--8+*2
Unknown token: TSub
``` 
Our error handler worked here ( i know its 1234 + (-56)), but lets just skip for now :) . 
Lets use different input this time .
``` bash
Parsing: 1/2+2*3

AST: ASTnode { op: Some(Amul), left: Some(ASTnode { op: Some(Aadd), left: Some(ASTnode { op: Some(Adiv), left: Some(ASTnode { op: Some(Aint), left: None, right: None, val
ue: 1 }), right: Some(ASTnode { op: Some(Aint), left: None, right: None, value: 2 }), value: 0 }), right: Some(ASTnode { op: Some(Aint), left: None, right: None, value: 2
 }), value: 0 }), right: Some(ASTnode { op: Some(Aint), left: None, right: None, value: 3 }), value: 0 }
Result: 6
```
Oh, my bad ,didnt show you the interpreter.gimme a  sec.

``` rust
use super::defs::ASTnode;
use super::defs::ASTtype;

pub fn interpret(ast: &ASTnode) -> i32{
    let left: i32;
    let right: i32;
    match &ast.op.as_ref().unwrap(){
        ASTtype::Aadd => {
            left = interpret(&ast.left.as_ref().unwrap());
            right = interpret(&ast.right.as_ref().unwrap());
            left + right
        }
        ASTtype::Asub => {
            left = interpret(&ast.left.as_ref().unwrap());
            right = interpret(&ast.right.as_ref().unwrap());
            left - right
        }
        ASTtype::Amul => {
            left = interpret(&ast.left.as_ref().unwrap());
            right = interpret(&ast.right.as_ref().unwrap());
           left * right
        }
        ASTtype::Adiv => {
            left = interpret(&ast.left.as_ref().unwrap());
            right = interpret(&ast.right.as_ref().unwrap());
            left / right
        }
        ASTtype::Aint => {
            ast.value
        }
        _ => {
            println!("Unknown node type");
            std::process::exit(1);
        }
    }
    

}
```

It's just a recursive function that evaluates given nodes and leaves . after the expression got evaluated, we can see the result is wrong, it should be 6.5 but its shows 6 here ,its due to us not dealing with floats yet . we will get there as we progress through . 

In the next part, we will teach the parser some mathematics. 

