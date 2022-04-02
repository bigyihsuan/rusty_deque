use std::{collections::VecDeque, io::Write};

mod evaluator;
mod lexer;
mod parser;
use evaluator::eval_value::*;

// tests
mod tests;

fn main() {
    // let input_str = String::from(
    //     "# given an int n (n -- n~)
    // {dup~ 2! rot~ <!}~ rot~ {pop! 1!}! {dup~ 1~ -~ 2! -1! {in~ *!}! rot~ for~}! rot~ ite~",
    // );
    // println!("{}", input_str);
    // let tokens = lexer::lex::tokenize_code(&input_str);
    // // println!("{:?}", &tokens);
    // let ast = parser::par::parse_tokens(&mut tokens.into_iter());
    // println!("{:#?}", ast);

    // possible command-line args
    // all of these are optional
    // no args: run the REPL
    // -f <filename>: read code from this file
    // -c <expr>: evaluate this expression
    // -p: print the stack after code exectution
    // -t: print the tokens after lexing
    // -a: print the AST after parsing

    // cli format:
    // cargo run [-p] [-t] [-a] [-f <filename>] [-c <expr>]

    let args = std::env::args();
    let mut args_iter = args.skip(1); // skip program name
    let mut filename = String::new();
    let mut expr = String::new();
    let mut print_stack = false;
    let mut print_tokens = false;
    let mut print_ast = false;

    // check for args
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-f" => {
                if let Some(arg) = args_iter.next() {
                    filename = arg;
                } else {
                    println!("-f requires an argument: filename");
                    print_usage();
                    std::process::exit(1);
                }
            }
            "-c" => {
                if let Some(arg) = args_iter.next() {
                    expr = arg;
                } else {
                    println!("-c requires an argument: code");
                    print_usage();
                    std::process::exit(1);
                }
            }
            "-p" => {
                print_stack = true;
            }
            "-t" => {
                print_tokens = true;
            }
            "-a" => {
                print_ast = true;
            }
            _ => {
                println!("unrecognized argument: {}", arg);
                print_usage();
                std::process::exit(1);
            }
        }
    }
    // if no args, run the repl
    if filename.is_empty() && expr.is_empty() {
        repl(print_tokens, print_ast, print_stack);
        std::process::exit(0);
    } else if filename.is_empty() {
        // if no filename, but expr, run expr
        if !expr.is_empty() {
            run_code(Option::None, expr, print_tokens, print_ast, print_stack);
            std::process::exit(0);
        } else {
            println!("no expression specified");
            print_usage();
            std::process::exit(1);
        }
    } else {
        // if filename, run file
        // read from file
        let contents = std::fs::read_to_string(filename).expect("file not found");
        run_code(Option::None, contents, print_tokens, print_ast, print_stack);
        std::process::exit(0);
    }
}

pub fn print_usage() {
    println!("usage: cargo run [-p] [-t] [-a] [-f <filename>] [-c <expr>]");
    println!("-p: print the stack after code exectution");
    println!("-t: print the tokens after lexing");
    println!("-a: print the AST after parsing");
    println!("-f <filename>: read code from this file");
    println!("-c <expr>: evaluate this expression");
}

pub fn run_code(
    deque: Option<VecDeque<Value>>,
    code: String,
    print_tokens: bool,
    print_ast: bool,
    print_stack: bool,
) -> VecDeque<Value> {
    // lex
    let tokens = lexer::lex::tokenize_code(&code);
    if print_tokens {
        println!("{:?}", &tokens);
    }
    // parse
    let ast = parser::par::parse_tokens(&mut tokens.into_iter());
    if print_ast {
        println!("{:#?}", ast);
    }
    // run
    let deque = evaluator::eval::run_ast(deque, ast);
    if print_stack {
        let mut out_str = "(".to_string();
        let mut deque_iter = deque.iter();
        for _ in 0..deque.len() {
            let val = deque_iter.next().unwrap();
            out_str.push_str(val.clone().to_string().as_str());
            out_str.push_str(", ");
        }
        out_str.push_str(")");
        println!("{}", out_str);
    }
    deque
}

pub fn repl(print_tokens: bool, print_ast: bool, print_stack: bool) {
    let mut deque: VecDeque<evaluator::eval_value::Value> = std::collections::VecDeque::new();
    loop {
        // print the prompt
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        // get input
        let mut input = String::new();
        let result = std::io::stdin().read_line(&mut input);
        match result {
            Ok(size) => {
                if size <= 0 {
                    println!("");
                    return;
                }
                deque = run_code(Some(deque), input, print_tokens, print_ast, print_stack);
            }
            Err(_) => {
                return;
            }
        }
    }
}
