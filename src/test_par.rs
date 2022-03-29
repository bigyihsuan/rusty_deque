#[cfg(test)]
mod tests {
    use crate::lexer::lex::*;
    use crate::lexer::lex_token::*;
    use crate::parser::par::*;
    use crate::parser::par_ast::*;

    #[test]
    fn test_par_hello_world() {
        let input_str = String::from("\"Hello World!\"~ ow~");
        let tokens = tokenize_code(&input_str);
        let code = parse_tokens(tokens);

        let expected = vec![
            Exec::Right(Op::Literal(Literal::List(vec![
                Literal::Char('H'),
                Literal::Char('e'),
                Literal::Char('l'),
                Literal::Char('l'),
                Literal::Char('o'),
                Literal::Char(' '),
                Literal::Char('W'),
                Literal::Char('o'),
                Literal::Char('r'),
                Literal::Char('l'),
                Literal::Char('d'),
                Literal::Char('!'),
            ]))),
            Exec::Right(Op::Instruction(String::from("ow"))),
        ];

        println!("{:?}", code);
        assert_eq!(code, expected);
    }

    #[test]
    #[should_panic(expected = "Parser Error: Unexpected token type")]
    fn test_par_invalid_literal() {
        let input_str = String::from("ow");
        let token = get_next_token(&input_str, 0, 0).0;
        parse_literal(token);
    }

    #[test]
    fn test_par_ints() {
        let ints = vec![
            String::from("1"),
            String::from("-1"),
            String::from("0"),
            String::from("-0"),
            String::from("123"),
            String::from("-123"),
            String::from("123456789"),
            String::from("-123456789"),
        ];

        let expected = vec![
            Literal::Int(1),
            Literal::Int(-1),
            Literal::Int(0),
            Literal::Int(0),
            Literal::Int(123),
            Literal::Int(-123),
            Literal::Int(123456789),
            Literal::Int(-123456789),
        ];

        for (i, input) in ints.iter().enumerate() {
            let token = get_next_token(&input, 0, 0).0;
            let literal = parse_literal(token);

            assert_eq!(expected[i], literal);
            println!("{:?}", literal);
        }
    }

    #[test]
    fn test_par_floats() {
        let floats = vec![
            String::from("1.0"),
            String::from("-1.0"),
            String::from("0.0"),
            String::from("-0.0"),
            String::from("123.0"),
            String::from("-123.0"),
            String::from("123456789.0"),
            String::from("-123456789.0"),
            String::from("1.23456789"),
            String::from("-1.23456789"),
        ];

        let expected = vec![
            Literal::Float(1.0),
            Literal::Float(-1.0),
            Literal::Float(0.0),
            Literal::Float(0.0),
            Literal::Float(123.0),
            Literal::Float(-123.0),
            Literal::Float(123456789.0),
            Literal::Float(-123456789.0),
            Literal::Float(1.23456789),
            Literal::Float(-1.23456789),
        ];

        for (i, input) in floats.iter().enumerate() {
            let token = get_next_token(&input, 0, 0).0;
            let literal = parse_literal(token);

            assert_eq!(expected[i], literal);
            println!("{:?}", literal);
        }
    }

    #[test]
    fn test_par_chars() {
        let chars = vec![
            String::from("'a'"),
            String::from("'X'"),
            String::from("'0'"),
            String::from("'!'"),
            String::from("'\\\"'"),
            String::from("'\\\''"),
            String::from("'\\n'"),
            String::from("'\\t'"),
            String::from("'\\r'"),
            String::from("'\\0'"),
            String::from("'\\\\'"),
        ];
        let expected = vec![
            Literal::Char('a'),
            Literal::Char('X'),
            Literal::Char('0'),
            Literal::Char('!'),
            Literal::Char('"'),
            Literal::Char('\''),
            Literal::Char('\n'),
            Literal::Char('\t'),
            Literal::Char('\r'),
            Literal::Char('\0'),
            Literal::Char('\\'),
        ];

        for (i, input) in chars.iter().enumerate() {
            println!("input: {:}", input);
            let token = get_next_token(&input, 0, 0).0;
            let literal = parse_literal(token);

            println!("{:?}\n", literal);
            assert_eq!(expected[i], literal);
        }
    }

    #[test]
    fn test_par_strings() {
        let strings = vec![
            String::from("\"Hello World!\""),
            String::from("\"\""), // empty string
            String::from("\"Hello\""),
            String::from("\"\\\"\""),   // string with escaped double quotes
            String::from("\"\\'\\'\""), // string with escaped single quotes
            String::from("\"Hello\\nWorld!\""),
            String::from("\"Hello\\n\\tWorld!\""),
        ];
        let expected = vec![
            Literal::List(
                String::from("Hello World!")
                    .chars()
                    .map(|c| Literal::Char(c))
                    .collect::<Vec<Literal>>(),
            ),
            Literal::List(
                String::from("")
                    .chars()
                    .map(|c| Literal::Char(c))
                    .collect::<Vec<Literal>>(),
            ),
            Literal::List(
                String::from("Hello")
                    .chars()
                    .map(|c| Literal::Char(c))
                    .collect::<Vec<Literal>>(),
            ),
            Literal::List(
                String::from("\"")
                    .chars()
                    .map(|c| Literal::Char(c))
                    .collect::<Vec<Literal>>(),
            ),
            Literal::List(
                String::from("\'\'")
                    .chars()
                    .map(|c| Literal::Char(c))
                    .collect::<Vec<Literal>>(),
            ),
            Literal::List(
                String::from("Hello\nWorld!")
                    .chars()
                    .map(|c| Literal::Char(c))
                    .collect::<Vec<Literal>>(),
            ),
            Literal::List(
                String::from("Hello\n\tWorld!")
                    .chars()
                    .map(|c| Literal::Char(c))
                    .collect::<Vec<Literal>>(),
            ),
        ];

        for (expect, input) in expected.iter().zip(strings.iter()) {
            let token = get_next_token(&input, 0, 0).0;
            let literal = parse_literal(token);

            assert_eq!(expect, &literal);
            println!("{:?}", literal);
        }
    }

    #[test]
    #[should_panic(expected = "Parser Error: Unrecognized character escape sequence")]
    fn test_par_invalid_char_escapes_for_char() {
        let input_str = String::from("'\\a'");
        let token = get_next_token(&input_str, 0, 0).0;
        parse_literal(token);
    }

    #[test]
    #[should_panic(expected = "Parser Error: Unrecognized character escape sequence")]
    fn test_par_invalid_char_escapes_for_string() {
        let input_str = String::from("\"\\a\"");
        let token = get_next_token(&input_str, 0, 0).0;
        parse_literal(token);
    }
}
