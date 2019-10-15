pub mod eval;
pub mod lexer;
pub mod node;
pub mod parser;
pub mod token;
pub mod util;
pub mod value;
use crate::eval::Evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let program = r#"
        def a(x,y)
            puts ("abd\t6")
            x+y
        end
        a(1,2)
        "wsnd\nferg"
        "#;
    println!("{}", program);
    let lexer = Lexer::new(program);
    match lexer.tokenize() {
        Err(err) => println!("ParseError: {:?}", err),
        Ok(result) => {
            for token in &result.tokens {
                println!("{}", token);
            }
            let mut parser = Parser::new(result);
            match parser.parse_program() {
                Ok(node) => {
                    println!("{}", node);
                    let mut eval = Evaluator::new(parser.source_info, parser.ident_table);
                    println!("result: {:?}", eval.eval_node(&node));
                }
                Err(err) => {
                    println!("ParseError: {:?}", err.kind);
                }
            }
        }
    }
}
