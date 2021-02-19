mod lexer;
mod lexer2;
mod parser;
mod util;

fn main() {
    // let code =
    //     "!{!dup !2 !rot <!} !rot {pop! 1!}! {!dup !1 !- 2! -1! {!in !*}! !rot !for}! !rot !ite";

    let code = "{![1, 2, 0.3, [4, '5', \"six\", 7.8,], 9,] ol!}!";
    let mut c = String::from(code);
    c.push(' ');
    let tokens = lexer::tokenize(c);
    let tokens2 = lexer2::tokenize(String::from(code));
    println!("Code: {}", String::from(code));
    // for t in tokens {
    //     println!("{:?}", t);
    // }
    // println!("");
    for t in tokens2 {
        println!("{:?}", t);
    }
    //parser::parse(tokens);
}
