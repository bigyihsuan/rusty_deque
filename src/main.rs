mod lexer;
mod tests;

fn main() {
    let input_str = String::from(
        "# given an int n (n -- n~)
    {dup~ 2! rot~ <!}~ rot~ {pop! 1!}! {dup~ 1~ -~ 2! -1! {in~ *!}! rot~ for~}! rot~ ite~",
    );
    println!("{}", input_str);
    let tokens = lexer::lex::tokenize_code(&input_str);
    for token in tokens {
        println!("{:?}", token);
    }
}
