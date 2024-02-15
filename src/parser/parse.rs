use super::defs::ASTnode;
use super::defs::ASTtype;
use crate::lexer::scan::Token;
use crate::lexer::scan::TokenType::*;
use crate::lexer::scan::Lexer;
pub fn parse(input: &str) {
    let mut lexer = Lexer::new(input.to_string());
    let token = lexer.scan();
    let ast = binexpr(&mut lexer, token);
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
    while token.token_type == TAdd || token.token_type == TSub || token.token_type == TMul || token.token_type == TDiv {
        let op = arithop(&token);
        let  right = primary(&lexer.scan());
        left = new_ast_node(op as i32, Box::new(left), Box::new(right), 0);
        token = lexer.scan();
    }
    left
}



pub fn new_ast_node(op: i32, left: Box<ASTnode>, right: Box<ASTnode>, value: i32) -> ASTnode {
    ASTnode {
        op: op,
        left: left,
        right: right,
        value: value,
    }
}

pub fn new_ast_leaf(value: i32) -> ASTnode {
    new_ast_node(0, Box::new(ASTnode::new()), Box::new(ASTnode::new()), value)
}

impl ASTnode {
    pub fn new() -> ASTnode {
        ASTnode {
            op: 0,
            left: Box::new(ASTnode::new()),
            right: Box::new(ASTnode::new()),
            value: 0,
        }
    }
}
pub fn new_ast_unary(op: i32, left: Box<ASTnode>, value: i32) -> ASTnode {
    new_ast_node(op, left, Box::new(ASTnode::new()), value)
}
