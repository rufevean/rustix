// abstract syntax tree for the language

pub struct ASTnode {
    op: i32,
    left: Box<ASTnode>,
    right: Box<ASTnode>,
    value: i32,
}
// AST node types

pub enum ASTtype {
    A_ADD,
    A_SUBTRACT,
    A_MULTIPLY,
    A_DIVIDE,
    A_INTLIT,
}


