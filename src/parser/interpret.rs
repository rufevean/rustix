use super::defs::ASTnode;
use super::defs::ASTtype;

pub fn interpret(ast: &ASTnode) -> i32{
    let mut left: i32;
    let mut right: i32;
    
    while precedence(&ast.op.as_ref().unwrap()) > precedence(&ast.op.as_ref().unwrap()){
        left = interpret(&ast.left.as_ref().unwrap());
        right = interpret(&ast.right.as_ref().unwrap());
    }

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

pub fn precedence(op: &ASTtype) -> i32 {
    match op {
        ASTtype::Aadd => 1,
        ASTtype::Asub => 1,
        ASTtype::Amul => 2,
        ASTtype::Adiv => 2,
        _ => 0
    }
}
