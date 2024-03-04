// abstract syntax tree for the language

#[derive(Debug,Copy)]
pub struct ASTnode {
    pub op: Option<ASTtype>,
    pub left: Option<Box<ASTnode>>,
    pub right: Option<Box<ASTnode>>,
    pub value: i32,
}
// AST node types
#[derive(Debug,Copy)]
pub enum ASTtype {
    Aadd,
    Asub,
    Amul,
    Adiv,
    Aint,
    Aunknown,
}

// build and return a new AST node
