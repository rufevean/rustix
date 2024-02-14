use crate::lexer::scan::Token;
pub fn parse(input_vector : &Vec<Token>){
    for token in input_vector {
        println!("{:?}", token);
    }
}
