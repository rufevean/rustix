// abstract syntax tree for the language

#[derive(Debug)]
pub struct ASTnode {
    pub op: i32,
    pub left: Box<ASTnode>,
    pub right: Box<ASTnode>,
    pub value: i32,
}
// AST node types

pub enum ASTtype {
    Aadd,
    Asub,
    Amul,
    Adiv,
    Aint,
    Aunknown,
}

// build and return a new AST node
