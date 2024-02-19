use super::defs::ASTnode;
use super::defs::ASTtype;

use super::interpret::interpret;

use crate::lexer::scan::Lexer;
use crate::lexer::scan::Token;
use crate::lexer::scan::TokenType::*;
pub fn parse(input: &str) {
    println!("Parsing: {}", input);
    let mut lexer = Lexer::new(input.to_string());
    let token = lexer.scan();
    let ast = binexpr(&mut lexer, token);
    println!("AST: {:?}", ast);
    let result = interpret(&ast);
    println!("Result: {}", result);
    println!("{:?}", ast);

}

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


