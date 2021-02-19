mod lexer;
mod parser;
mod util;

fn main() {
    let code =
        "!{!dup !2 !rot <!} !rot {pop! 1!}! {!dup !1 !- 2! -1! {!in !*}! !rot !for}! !rot !ite";
    let mut c = String::from(code);
    c.push(' ');
    let tokens = lexer::tokenize(c);
    println!("Code: {:?}", code);
    println!("Tokens: {:#?}", tokens);
}
