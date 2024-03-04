use crate::parser::defs::ASTnode;
use crate::parser::defs::ASTtype;
use super::regfn::*;


pub static mut REGISTERS : Vec<&str> = vec!["%r8","%r9","%r10","%r11"];



pub fn gen(&ast: &ASTnode) -> i32{
    let mut  leftreg : i32;
    let mut rightreg : i32;

    
    match &ast.op.as_ref().unwrap(){
        ASTtype::Aadd => {
            leftreg = gen(&ast.left.as_ref().unwrap());
            rightreg = gen(&ast.right.as_ref().unwrap());
            cgadd(leftreg,rightreg)
        }
        ASTtype::Asub => {
            leftreg = gen(&ast.left.as_ref().unwrap());
            rightreg = gen(&ast.right.as_ref().unwrap());
            cgadd(leftreg,rightreg)
        }
        ASTtype::Amul => {
            leftreg = gen(&ast.left.as_ref().unwrap());
            rightreg = gen(&ast.right.as_ref().unwrap());
            cgadd(leftreg,rightreg)
        }
        ASTtype::Adiv => {
            leftreg = gen(&ast.left.as_ref().unwrap());
            rightreg = gen(&ast.right.as_ref().unwrap());
            cgadd(leftreg,rightreg)
        }
        ASTtype::Aint => {
            cgload(ast.value)
        }
        _ => {
            println!("Unknown node type");
            std::process::exit(1);
        }
    }
    
}


fn generatecode(&ast: &ASTnode){
    let mut reg : i32;
    cgpreamble();
    reg = gen(&ast);
    cgprintint(reg);
    cgpostamble();
}
