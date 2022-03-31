#[cfg(test)]
mod tests {
    use crate::evaluator::eval::*;
    use crate::lexer::lex::*;
    use crate::parser::par::*;
    #[test]
    fn test_eval_literals() {
        let input_str = String::from("[1.2, 'a', [true, 3], -4]~ {ol~}~");
        let tokens = tokenize_code(&input_str);
        let ast = parse_tokens(&mut tokens.into_iter());
        println!("{:#?}", ast);
        run_ast(ast);
    }

    #[test]
    fn test_eval_hello_world() {
        let input_str = String::from("\"Hello, World!\"~ ol~");
        let tokens = tokenize_code(&input_str);
        let ast = parse_tokens(&mut tokens.into_iter());
        println!("{:#?}", ast);
        run_ast(ast);
    }

    #[test]
    fn test_eval_dup() {
        let input_str = String::from("1~ dup~");
        let tokens = tokenize_code(&input_str);
        let ast = parse_tokens(&mut tokens.into_iter());
        println!("{:#?}", ast);
        run_ast(ast);
    }
}
