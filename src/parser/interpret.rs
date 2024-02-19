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
