#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::evaluator::eval::*;
    use crate::evaluator::eval_value::Value;
    use crate::lexer::lex::*;
    use crate::parser::par::*;
    #[test]
    fn test_eval_literals() {
        let input_str = String::from("[1.2, 'a', [true, 3], -4]~ {ol~}~");
        let tokens = tokenize_code(&input_str);
        let ast = parse_tokens(&mut tokens.into_iter());
        println!("{:#?}", ast);
        run_ast(Option::None, ast);
    }

    #[test]
    fn test_eval_hello_world() {
        let input_str = String::from("\"Hello, World!\"~ ol~");
        let tokens = tokenize_code(&input_str);
        let ast = parse_tokens(&mut tokens.into_iter());
        // println!("{:#?}", ast);
        run_ast(Option::None, ast);
    }

    #[test]
    fn test_eval_dup() {
        let input_str = String::from("1~ dup~");
        let tokens = tokenize_code(&input_str);
        let ast = parse_tokens(&mut tokens.into_iter());
        // println!("{:#?}", ast);
        let deque = run_ast(Option::None, ast);
        let expected = VecDeque::from(vec![Value::Int(1), Value::Int(1)]);
        assert_eq!(deque, expected);
    }

    #[test]
    #[should_panic]
    fn test_eval_invalid_types_for_add() {
        let input_str = String::from("1.2! 'a'~ +!");
        let tokens = tokenize_code(&input_str);
        let ast = parse_tokens(&mut tokens.into_iter());
        // println!("{:#?}", ast);
        run_ast(Option::None, ast);
    }

    #[test]
    fn test_eval_add_int_only() {
        let inputs = vec![
            String::from("1~ 2~ +!"),
            String::from("-1~ 2~ +!"),
            String::from("1~ -2~ +!"),
            String::from("-1~ -2~ +!"),
        ];

        let expected = vec![
            VecDeque::from(vec![Value::Int(3)]),
            VecDeque::from(vec![Value::Int(1)]),
            VecDeque::from(vec![Value::Int(-1)]),
        ];

        for (input, expected) in inputs.iter().zip(expected.iter()) {
            let tokens = tokenize_code(input);
            let ast = parse_tokens(&mut tokens.into_iter());
            // println!("{:#?}", ast);
            let deque = run_ast(Option::None, ast);
            assert_eq!(deque, *expected);
        }
    }
    // #[test]
    // fn test_eval_add_mixed() {
    //     let inputs = vec![
    //         String::from("1.1~ 2~ +!"),
    //         String::from("1~ 2.2~ +!"),
    //         String::from("-1.1~ 2~ +!"),
    //         String::from("-1~ 2.2~ +!"),
    //         String::from("1.1~ -2~ +!"),
    //         String::from("1~ -2.2~ +!"),
    //         String::from("-1.1~ -2~ +!"),
    //         String::from("-1~ -2.2~ +!"),
    //     ];

    //     let expected = vec![
    //         VecDeque::from(vec![Value::Float(3.1)]),
    //         VecDeque::from(vec![Value::Float(3.2)]),
    //         VecDeque::from(vec![Value::Float(0.9)]),
    //         VecDeque::from(vec![Value::Float(1.2)]),
    //         VecDeque::from(vec![Value::Float(-0.9)]),
    //         VecDeque::from(vec![Value::Float(-1.2)]),
    //         VecDeque::from(vec![Value::Float(-3.3)]),
    //         VecDeque::from(vec![Value::Float(-3.3)]),
    //     ];

    //     for (input, expected) in inputs.iter().zip(expected.iter()) {
    //         let tokens = tokenize_code(input);
    //         let ast = parse_tokens(&mut tokens.into_iter());
    //         // println!("{:#?}", ast);
    //         let deque = run_ast(Option::None, ast);
    //         assert_eq!(deque, *expected);
    //     }
    // }

    // #[test]
    // fn test_eval_add_float_only() {
    //     let inputs = vec![
    //         String::from("1.1~ 2.2~ +!"),
    //         String::from("-1.1~ 2.2~ +!"),
    //         String::from("1.1~ -2.2~ +!"),
    //         String::from("-1.1~ -2.2~ +!"),
    //     ];

    //     let expected = vec![
    //         VecDeque::from(vec![Value::Float(3.3)]),
    //         VecDeque::from(vec![Value::Float(1.1)]),
    //         VecDeque::from(vec![Value::Float(-1.1)]),
    //         VecDeque::from(vec![Value::Float(-3.3)]),
    //     ];

    //     for (input, expected) in inputs.iter().zip(expected.iter()) {
    //         let tokens = tokenize_code(input);
    //         let ast = parse_tokens(&mut tokens.into_iter());
    //         // println!("{:#?}", ast);
    //         let deque = run_ast(Option::None, ast);
    //         assert_eq!(deque, *expected);
    //     }
    // }
}
