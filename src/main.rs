mod evaluator;
mod lexer;
mod parser;

fn main() {
    // let code = "{dup~ 2! rot~ <!}~ rot~ {pop! 1!}! {dup~ 1~ -~ 2! -1! {in~ *!}! rot~ for~}! rot~ ite~";

    let code = "{ alpha! { beta! }~ delta! }!";
    // let code = "[1, 2, 0.3, [4, '5', \"six\", 7.8,], 9,]~ ol!";
    // let code = "{{a! b! c!}! {d! e! f!}!}!";
    // let code = "{ia~ ol~}~ loop~";
    // let code = "ia~ ol~";

    // let mut c = String::from(code);
    // c.push(' ');
    // let tokens = lexer::lex::tokenize(c);
    // println!("Code: {}", String::from(code));
    // for t in &tokens {
    //     println!("{:?} {} @ {}", t.token_type, t.string, t.tok_index);
    // }

    // let ast = parser::par::parse_tokens(tokens);
    // println!("ast: {:#?}", &ast);

    // use evaluator::visit::*;
    // let mut tree_printer = evaluator::tree_print::TreePrinter::new();
    // println!("{}", tree_printer.visit_code(ast));

    repl::repl();
}

pub mod repl {
    use crate::evaluator;
    use crate::evaluator::visit::VisitorRun;
    use crate::lexer;
    use crate::parser;
    use std::io::{stdin, stdout, Write};

    pub fn repl() {
        let mut stack = evaluator::tree_evaluator::Evaluator::new();
        loop {
            let mut input = String::new();
            print!(">>> ");
            let _ = stdout().flush();
            stdin().read_line(&mut input).expect("Enter some code");
            let tokens = lexer::lex::tokenize(input);
            let ast = parser::par::parse_tokens(tokens);
            stack.visit_code(ast);
            println!("{:?}", stack);
        }
    }
}
