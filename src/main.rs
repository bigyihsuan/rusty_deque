mod lexer;

fn main() {
    let tokens = lexer::tokenize(String::from(
        "!{!dup !2 !rot <!} !rot {pop! 1!}! {!dup !1 !- 2! -1! {!in !*}! !rot !for}! !rot !ite",
    ));
    println!("Tokens: {:?}", tokens);
}
