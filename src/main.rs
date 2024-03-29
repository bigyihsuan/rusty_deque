use std::collections::VecDeque;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod evaluator;
mod lexer;
mod parser;
use evaluator::eval_value::*;

// tests
mod tests;

fn main() {
    // possible command-line args
    // all of these are optional
    // no args: run the REPL
    // -f <filename>: read code from this file
    // -c <expr>: evaluate this expression
    // -p: print the stack after code exectution
    // -t: print the tokens after lexing
    // -a: print the AST after parsing

    // cli format:
    // cargo run -- [-p] [-t] [-a] [-f <filename>] [-c <expr>]

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
            let result = run_code(Option::None, expr, print_tokens, print_ast, print_stack);
            match result {
                Ok(_) => std::process::exit(0),
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
        } else {
            println!("no expression specified");
            print_usage();
            std::process::exit(1);
        }
    } else {
        // if filename, run file
        // read from file
        let contents = std::fs::read_to_string(filename).expect("file not found");
        let contents = contents.trim().to_string();
        let result = run_code(Option::None, contents, print_tokens, print_ast, print_stack);
        match result {
            Ok(_) => std::process::exit(0),
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            }
        }
    }
}

pub fn print_usage() {
    println!("usage: cargo run -- [-p] [-t] [-a] [-f <filename>] [-c <expr>]");
    println!("-p: print the stack after code execution");
    println!("-t: print the tokens after lexing");
    println!("-a: print the AST after parsing");
    println!("-f <filename>: read code from this file");
    println!("-c <expr>: evaluate this expression");
    println!("(no args): run the REPL");
}

pub fn run_code(
    deque: Option<VecDeque<Value>>,
    code: String,
    print_tokens: bool,
    print_ast: bool,
    print_stack: bool,
) -> Result<VecDeque<Value>, String> {
    // lex
    let tokens = lexer::lex::tokenize_code(&code);
    if print_tokens {
        println!("{:?}", &tokens);
    }
    // parse
    let ast = parser::par::parse_tokens(&mut tokens.into_iter());
    match ast {
        Ok(ast) => {
            if print_ast {
                println!("{:#?}", ast);
            }
            // run
            let deque = evaluator::eval::run_ast(deque, ast)?;
            if print_stack {
                print_deque(&deque);
            }
            Ok(deque)
        }
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    }
}

pub fn print_deque(deque: &VecDeque<Value>) {
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

pub fn repl(print_tokens: bool, print_ast: bool, print_stack: bool) {
    let mut deque: VecDeque<evaluator::eval_value::Value> = std::collections::VecDeque::new();

    // adapted from the example code on https://github.com/kkawakam/rustyline
    let mut rl = Editor::<()>::new();
    loop {
        print!("");
        let readline = rl.readline(">>> ");
        match readline {
            Ok(input) => {
                rl.add_history_entry(input.as_str());
                if input.len() <= 0 {
                    println!("");
                    break;
                }
                let code_result = run_code(
                    Some(deque.clone()),
                    input,
                    print_tokens,
                    print_ast,
                    print_stack,
                );
                match code_result {
                    Ok(new_deque) => {
                        deque = new_deque;
                        // println!();
                    }
                    Err(e) => {
                        println!("{}", e);
                        if print_stack {
                            print_deque(&deque);
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
