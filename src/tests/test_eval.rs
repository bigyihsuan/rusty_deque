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
        // println!("{:#?}", ast.unwrap());
        run_ast(Option::None, ast.unwrap());
    }

    #[test]
    fn test_eval_hello_world() {
        let input_str = String::from("\"Hello, World!\"~ ol~");
        let tokens = tokenize_code(&input_str);
        let ast = parse_tokens(&mut tokens.into_iter());
        // println!("{:#?}", ast.unwrap());
        run_ast(Option::None, ast.unwrap());
    }

    #[test]
    fn test_eval_dup() {
        let input_str = String::from("1~ dup~");
        let tokens = tokenize_code(&input_str);
        let ast = parse_tokens(&mut tokens.into_iter());
        // println!("{:#?}", ast.unwrap());
        let deque = run_ast(Option::None, ast.unwrap());
        let expected = VecDeque::from(vec![Value::Int(1), Value::Int(1)]);
        assert_eq!(deque, expected);
    }

    // #[test]
    // fn test_eval_invalid_types_for_add() -> Result<(), String> {
    //     let input_str = String::from("1.2! 'a'~ +!");
    //     let tokens = tokenize_code(&input_str);
    //     let ast = parse_tokens(&mut tokens.into_iter());
    //     match ast {
    //         Ok(_) => {
    //             let code_result = run_ast(Option::None, ast.unwrap());
    //             match code_result {
    //                 Ok(_) => Err("Expected error".to_string()),
    //                 Err(e) => {
    //                     assert_eq!(e, String::from("invalid operands for addition"));
    //                     Ok(())
    //                 }
    //             }
    //         }
    //         Err(e) => {
    //             panic!("{}", e);
    //         }
    //     }
    // }

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
            // println!("{:#?}", ast.unwrap());
            let deque = run_ast(Option::None, ast.unwrap());
            assert_eq!(deque, *expected);
        }
    }

    // get around the pesky floating point inaccuracies
    // https://stackoverflow.com/a/28656825/8143168
    fn round_to_nth_decimal_place(num: f64, n: usize) -> f64 {
        let ten_to_nth = 10_f64.powi(n as i32);
        (num * ten_to_nth).round() / ten_to_nth
    }

    #[test]
    fn test_eval_add_mixed() {
        let inputs = vec![
            String::from("1.1~ 2~ +!"),
            String::from("1~ 2.2~ +!"),
            String::from("-1.1~ 2~ +!"),
            String::from("-1~ 2.2~ +!"),
            String::from("1.1~ -2~ +!"),
            String::from("1~ -2.2~ +!"),
            String::from("-1.1~ -2~ +!"),
            String::from("-1~ -2.2~ +!"),
        ];

        let expected = vec![
            VecDeque::from(vec![Value::Float(3.1)]),
            VecDeque::from(vec![Value::Float(3.2)]),
            VecDeque::from(vec![Value::Float(0.9)]),
            VecDeque::from(vec![Value::Float(1.2)]),
            VecDeque::from(vec![Value::Float(-0.9)]),
            VecDeque::from(vec![Value::Float(-1.2)]),
            VecDeque::from(vec![Value::Float(-3.1)]),
            VecDeque::from(vec![Value::Float(-3.2)]),
        ];

        for (input, expected) in inputs.iter().zip(expected.iter()) {
            let tokens = tokenize_code(input);
            let ast = parse_tokens(&mut tokens.into_iter());
            let deque = run_ast(Option::None, ast.unwrap());
            let deque = deque
                .into_iter()
                .map(|v| {
                    if let Value::Float(f) = v {
                        Value::Float(round_to_nth_decimal_place(f, 1))
                    } else {
                        v
                    }
                })
                .collect::<VecDeque<Value>>();
            assert_eq!(deque, *expected);
        }
    }

    #[test]
    fn test_eval_add_float_only() {
        let inputs = vec![
            String::from("1.1~ 2.2~ +!"),
            String::from("-1.1~ 2.2~ +!"),
            String::from("1.1~ -2.2~ +!"),
            String::from("-1.1~ -2.2~ +!"),
        ];

        let expected = vec![
            VecDeque::from(vec![Value::Float(3.3)]),
            VecDeque::from(vec![Value::Float(1.1)]),
            VecDeque::from(vec![Value::Float(-1.1)]),
            VecDeque::from(vec![Value::Float(-3.3)]),
        ];

        for (input, expected) in inputs.iter().zip(expected.iter()) {
            let tokens = tokenize_code(input);
            let ast = parse_tokens(&mut tokens.into_iter());
            let deque = run_ast(Option::None, ast.unwrap());
            let deque = deque
                .into_iter()
                .map(|v| {
                    if let Value::Float(f) = v {
                        Value::Float(round_to_nth_decimal_place(f, 1))
                    } else {
                        v
                    }
                })
                .collect::<VecDeque<Value>>();
            assert_eq!(deque, *expected);
        }
    }

    // #[test]
    // #[should_panic]
    // fn test_eval_invalid_types_for_sub() {
    //     let input_str = String::from("1.2! 'a'~ -!");
    //     let tokens = tokenize_code(&input_str);
    //     let ast = parse_tokens(&mut tokens.into_iter());
    //     // println!("{:#?}", ast.unwrap());
    //     run_ast(Option::None, ast.unwrap());
    // }

    #[test]
    fn test_eval_sub_int_only() {
        let inputs = vec![
            String::from("1~ 2~ -~"),
            String::from("-1~ 2~ -~"),
            String::from("1~ -2~ -~"),
            String::from("-1~ -2~ -~"),
        ];

        let expected = vec![
            VecDeque::from(vec![Value::Int(1)]),  // 2-1
            VecDeque::from(vec![Value::Int(3)]),  // 2- -1
            VecDeque::from(vec![Value::Int(-3)]), // -2 -1
            VecDeque::from(vec![Value::Int(-1)]), // -2 - -1
        ];

        for (input, expected) in inputs.iter().zip(expected.iter()) {
            let tokens = tokenize_code(input);
            let ast = parse_tokens(&mut tokens.into_iter());
            // println!("{:#?}", ast.unwrap());
            let deque = run_ast(Option::None, ast.unwrap());
            assert_eq!(deque, *expected);
        }
    }

    #[test]
    fn test_eval_sub_mixed() {
        let inputs = vec![
            String::from("1.1~ 2~ -!"),
            String::from("1~ 2.2~ -!"),
            String::from("-1.1~ 2~ -!"),
            String::from("-1~ 2.2~ -!"),
            String::from("1.1~ -2~ -!"),
            String::from("1~ -2.2~ -!"),
            String::from("-1.1~ -2~ -!"),
            String::from("-1~ -2.2~ -!"),
        ];

        let expected = vec![
            VecDeque::from(vec![Value::Float(-0.9)]), // 1.1 - 2
            VecDeque::from(vec![Value::Float(-1.2)]), // 1 - 2.2
            VecDeque::from(vec![Value::Float(-3.1)]), // -1.1 - 2
            VecDeque::from(vec![Value::Float(-3.2)]), // -1 - 2.2
            VecDeque::from(vec![Value::Float(3.1)]),  // 1.1 - -2
            VecDeque::from(vec![Value::Float(3.2)]),  // 1 - -2.2
            VecDeque::from(vec![Value::Float(0.9)]),  // -1.1 - -2
            VecDeque::from(vec![Value::Float(1.2)]),  // -1 - -2.2
        ];

        for (input, expected) in inputs.iter().zip(expected.iter()) {
            let tokens = tokenize_code(input);
            let ast = parse_tokens(&mut tokens.into_iter());
            let deque = run_ast(Option::None, ast.unwrap());
            let deque = deque
                .into_iter()
                .map(|v| {
                    if let Value::Float(f) = v {
                        Value::Float(round_to_nth_decimal_place(f, 1))
                    } else {
                        v
                    }
                })
                .collect::<VecDeque<Value>>();
            assert_eq!(deque, *expected);
        }
    }

    #[test]
    fn test_eval_sub_float_only() {
        let inputs = vec![
            String::from("1.1~ 2.2~ -!"),
            String::from("-1.1~ 2.2~ -!"),
            String::from("1.1~ -2.2~ -!"),
            String::from("-1.1~ -2.2~ -!"),
        ];

        let expected = vec![
            VecDeque::from(vec![Value::Float(-1.1)]), // 1.1 - 2.2
            VecDeque::from(vec![Value::Float(-3.3)]), // -1.1 - 2.2
            VecDeque::from(vec![Value::Float(3.3)]),  // 1.1 - -2.2
            VecDeque::from(vec![Value::Float(1.1)]),  // -1.1 - -2.2
        ];

        for (input, expected) in inputs.iter().zip(expected.iter()) {
            let tokens = tokenize_code(input);
            let ast = parse_tokens(&mut tokens.into_iter());
            let deque = run_ast(Option::None, ast.unwrap());
            let deque = deque
                .into_iter()
                .map(|v| {
                    if let Value::Float(f) = v {
                        Value::Float(round_to_nth_decimal_place(f, 1))
                    } else {
                        v
                    }
                })
                .collect::<VecDeque<Value>>();
            assert_eq!(deque, *expected);
        }
    }
}
