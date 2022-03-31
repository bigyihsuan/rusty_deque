// tests
mod test_lex;
mod test_par;

mod lexer;
mod parser;

fn main() {
    let input_str = String::from(
        "# given an int n (n -- n~)
    {dup~ 2! rot~ <!}~ rot~ {pop! 1!}! {dup~ 1~ -~ 2! -1! {in~ *!}! rot~ for~}! rot~ ite~",
    );
    println!("{}", input_str);
    let tokens = lexer::lex::tokenize_code(&input_str);
    // println!("{:?}", &tokens);
    let ast = parser::par::parse_tokens(&mut tokens.into_iter());
    println!("{:#?}", ast);
}
