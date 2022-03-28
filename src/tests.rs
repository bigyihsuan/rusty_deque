#[cfg(test)]
mod tests {
    use crate::lexer::lex::*;
    use crate::lexer::lex_token::*;

    #[test]
    fn test_lex_const_int() {
        println!("Testing tokenizing ConstInt {}", 0);
        assert_eq!(
            get_next_token(&String::from("0"), 0, 0).0,
            Token {
                token_type: TokenType::ConstInt,
                lexeme: String::from("0"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 1,
            }
        );

        println!("Testing tokenizing ConstInt {}", 123);
        assert_eq!(
            get_next_token(&String::from("123"), 0, 0).0,
            Token {
                token_type: TokenType::ConstInt,
                lexeme: String::from("123"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 3,
            }
        );

        println!("Testing tokenizing ConstInt {}", -123);
        assert_eq!(
            get_next_token(&String::from("-123"), 0, 0).0,
            Token {
                token_type: TokenType::ConstInt,
                lexeme: String::from("-123"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 4,
            }
        );
    }

    #[test]
    fn test_lex_const_float() {
        println!("Testing tokenizing ConstFloat {}", 0.0);
        assert_eq!(
            get_next_token(&String::from("0.0"), 0, 0).0,
            Token {
                token_type: TokenType::ConstFloat,
                lexeme: String::from("0.0"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 3,
            }
        );

        println!("Testing tokenizing ConstFloat {}", 123.123);
        assert_eq!(
            get_next_token(&String::from("123.123"), 0, 0).0,
            Token {
                token_type: TokenType::ConstFloat,
                lexeme: String::from("123.123"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 7,
            }
        );

        println!("Testing tokenizing ConstFloat {}", -123.123);
        assert_eq!(
            get_next_token(&String::from("-123.123"), 0, 0).0,
            Token {
                token_type: TokenType::ConstFloat,
                lexeme: String::from("-123.123"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 8,
            }
        );

        println!("Testing tokenizing ConstFloat {}", 0.123);
        assert_eq!(
            get_next_token(&String::from("0.123"), 0, 0).0,
            Token {
                token_type: TokenType::ConstFloat,
                lexeme: String::from("0.123"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 5,
            }
        );

        println!("Testing failing ConstFloat {}", "0.123.123");
        assert_eq!(
            get_next_token(&String::from("0.123.123"), 0, 0).0,
            Token {
                token_type: TokenType::Error,
                lexeme: String::from("0.123"),
                error_msg: String::from(
                    "Invalid float literal: Floats cannot contain multiple decimal points"
                ),
                line: 0,
                start: 0,
                end: 5,
            }
        );

        println!("Testing missing decimal ConstFloat {}", "1234.");
        assert_eq!(
            get_next_token(&String::from("1234."), 0, 0).0,
            Token {
                token_type: TokenType::Error,
                lexeme: String::from("1234."),
                error_msg: String::from("Invalid float literal: missing decimal portion"),
                line: 0,
                start: 0,
                end: 5,
            }
        );
    }

    #[test]
    fn test_lex_const_char() {
        println!("Testing tokenizing ConstChar {}", 'a');
        assert_eq!(
            get_next_token(&String::from("'a'"), 0, 0).0,
            Token {
                token_type: TokenType::ConstChar,
                lexeme: String::from("'a'"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 3,
            }
        );

        println!("Testing tokenizing escaped ConstChar {}", "'\\n'");
        assert_eq!(
            get_next_token(&String::from("'\\n'"), 0, 0).0,
            Token {
                token_type: TokenType::ConstChar,
                lexeme: String::from("'\\n'"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 4,
            }
        );

        println!("Testing tokenizing unclosed ConstChar {}", "'a");
        assert_eq!(
            get_next_token(&String::from("'a"), 0, 0).0,
            Token {
                token_type: TokenType::Error,
                lexeme: String::from("'a"),
                error_msg: String::from("Invalid char literal: Unterminated character constant"),
                line: 0,
                start: 0,
                end: 2,
            }
        );

        println!("Testing tokenizing too-long ConstChar {}", "'ab'");
        assert_eq!(
            get_next_token(&String::from("'ab'"), 0, 0).0,
            Token {
                token_type: TokenType::Error,
                lexeme: String::from("'ab'"),
                error_msg: String::from("Invalid char literal: Character constant too long"),
                line: 0,
                start: 0,
                end: 4,
            }
        );
    }

    #[test]
    fn test_lex_const_string() {
        println!("Testing tokenizing ConstString \"{}\"", "a");
        assert_eq!(
            get_next_token(&String::from("\"a\""), 0, 0).0,
            Token {
                token_type: TokenType::ConstString,
                lexeme: String::from("\"a\""),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 3,
            }
        );

        println!("Testing tokenizing ConstString \"{}\"", "abc");
        assert_eq!(
            get_next_token(&String::from("\"abc\""), 0, 0).0,
            Token {
                token_type: TokenType::ConstString,
                lexeme: String::from("\"abc\""),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 5,
            }
        );

        println!("Testing tokenizing ConstString \"{}\"", "abcdef");
        assert_eq!(
            get_next_token(&String::from("\"abcdef\""), 0, 0).0,
            Token {
                token_type: TokenType::ConstString,
                lexeme: String::from("\"abcdef\""),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 8,
            }
        );

        println!(
            "Testing tokenizing ConstString \"{}\"",
            "abcd efghijklmnopqrst uvwxyz"
        );
        assert_eq!(
            get_next_token(&String::from("\"abcd efghijklmnopqrst uvwxyz\""), 0, 0).0,
            Token {
                token_type: TokenType::ConstString,
                lexeme: String::from("\"abcd efghijklmnopqrst uvwxyz\""),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 30,
            }
        );

        println!(
            "Testing tokenizing multiline ConstString \"{}\"",
            "abcdefghijklm\nopqrstuvwxyz\n0123456789"
        );
        assert_eq!(
            get_next_token(
                &String::from("\"abcdefghijklm\nopqrstuvwxyz\n0123456789\""),
                0,
                0
            )
            .0,
            Token {
                token_type: TokenType::ConstString,
                lexeme: String::from("\"abcdefghijklm\nopqrstuvwxyz\n0123456789\""),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 39,
            }
        );

        println!(
            "Testing tokenizing unterminated ConstString \"{}\"",
            "\"this is a test"
        );
        assert_eq!(
            get_next_token(&String::from("\"this is a test"), 0, 0).0,
            Token {
                token_type: TokenType::Error,
                lexeme: String::from("\"this is a test"),
                error_msg: String::from("Invalid string literal: Unterminated string constant"),
                line: 0,
                start: 0,
                end: 15,
            }
        );
    }

    #[test]
    fn test_lex_instr() {
        println!("Testing tokenizing Instr {}", "instr");
        assert_eq!(
            get_next_token(&String::from("instr"), 0, 0).0,
            Token {
                token_type: TokenType::Instr,
                lexeme: String::from("instr"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 5,
            }
        );

        println!("Testing tokenizing Instr {}", "true");
        assert_eq!(
            get_next_token(&String::from("true"), 0, 0).0,
            Token {
                token_type: TokenType::Instr,
                lexeme: String::from("true"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 4,
            }
        );

        println!("Testing tokenizing Instr {}", "false");

        println!("Testing tokenizing Instr {}", "-");
        assert_eq!(
            get_next_token(&String::from("-"), 0, 0).0,
            Token {
                token_type: TokenType::Instr,
                lexeme: String::from("-"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 1,
            }
        );

        println!("Testing tokenizing Instr {}", "+");
        assert_eq!(
            get_next_token(&String::from("+"), 0, 0).0,
            Token {
                token_type: TokenType::Instr,
                lexeme: String::from("+"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 1,
            }
        );

        println!("Testing tokenizing Instr {}", "dup");
        assert_eq!(
            get_next_token(&String::from("dup"), 0, 0).0,
            Token {
                token_type: TokenType::Instr,
                lexeme: String::from("dup"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 3,
            }
        );

        println!("Testing tokenizing Instr {}", "in");
        assert_eq!(
            get_next_token(&String::from("in"), 0, 0).0,
            Token {
                token_type: TokenType::Instr,
                lexeme: String::from("in"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 2,
            }
        );

        println!(
            "Testing tokenizing Instr {}",
            "this_isnt_an_instr_but_it_should_work"
        );
        assert_eq!(
            get_next_token(&String::from("this_isnt_an_instr_but_it_should_work"), 0, 0).0,
            Token {
                token_type: TokenType::Instr,
                lexeme: String::from("this_isnt_an_instr_but_it_should_work"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 37,
            }
        );
    }

    #[test]
    fn test_lex_single_char_tokens() {
        println!("Testing tokenizing Bang");
        assert_eq!(
            get_next_token(&String::from("!"), 0, 0).0,
            Token {
                token_type: TokenType::Bang,
                lexeme: String::from("!"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 1,
            }
        );

        println!("Testing tokenizing Tilde");
        assert_eq!(
            get_next_token(&String::from("~"), 0, 0).0,
            Token {
                token_type: TokenType::Tilde,
                lexeme: String::from("~"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 1,
            }
        );

        println!("Testing tokenizing Comma");
        assert_eq!(
            get_next_token(&String::from(","), 0, 0).0,
            Token {
                token_type: TokenType::Comma,
                lexeme: String::from(","),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 1,
            }
        );

        println!("Testing tokenizing LeftCurly");
        assert_eq!(
            get_next_token(&String::from("{"), 0, 0).0,
            Token {
                token_type: TokenType::LeftCurly,
                lexeme: String::from("{"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 1,
            }
        );

        println!("Testing tokenizing RightCurly");
        assert_eq!(
            get_next_token(&String::from("}"), 0, 0).0,
            Token {
                token_type: TokenType::RightCurly,
                lexeme: String::from("}"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 1,
            }
        );

        println!("Testing tokenizing LeftSquare");
        assert_eq!(
            get_next_token(&String::from("["), 0, 0).0,
            Token {
                token_type: TokenType::LeftSquare,
                lexeme: String::from("["),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 1,
            }
        );

        println!("Testing tokenizing RightSquare");
        assert_eq!(
            get_next_token(&String::from("]"), 0, 0).0,
            Token {
                token_type: TokenType::RightSquare,
                lexeme: String::from("]"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 1,
            }
        );
    }

    #[test]
    fn test_lex_multiple_tokens() {
        println!("Testing tokenizing multiple single-character tokens");

        let expected = vec![
            Token {
                token_type: TokenType::Bang,
                lexeme: String::from("!"),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 1,
            },
            Token {
                token_type: TokenType::Tilde,
                lexeme: String::from("~"),
                error_msg: String::new(),
                line: 0,
                start: 1,
                end: 2,
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: String::from(","),
                error_msg: String::new(),
                line: 0,
                start: 2,
                end: 3,
            },
            Token {
                token_type: TokenType::LeftCurly,
                lexeme: String::from("{"),
                error_msg: String::new(),
                line: 0,
                start: 3,
                end: 4,
            },
            Token {
                token_type: TokenType::RightCurly,
                lexeme: String::from("}"),
                error_msg: String::new(),
                line: 0,
                start: 4,
                end: 5,
            },
            Token {
                token_type: TokenType::LeftSquare,
                lexeme: String::from("["),
                error_msg: String::new(),
                line: 0,
                start: 5,
                end: 6,
            },
            Token {
                token_type: TokenType::RightSquare,
                lexeme: String::from("]"),
                error_msg: String::new(),
                line: 0,
                start: 6,
                end: 7,
            },
        ];

        let input_str = String::from("!~,{}[]");
        assert_eq!(expected, parse_code(&input_str));

        println!("Testing tokenizing list literal");
        let expected = vec![
            Token {
                token_type: TokenType::LeftSquare,
                lexeme: String::from("["),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 1,
            },
            Token {
                token_type: TokenType::ConstInt,
                lexeme: String::from("1"),
                error_msg: String::new(),
                line: 0,
                start: 1,
                end: 2,
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: String::from(","),
                error_msg: String::new(),
                line: 0,
                start: 2,
                end: 3,
            },
            Token {
                token_type: TokenType::ConstFloat,
                lexeme: String::from("2.2"),
                error_msg: String::new(),
                line: 0,
                start: 4,
                end: 7,
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: String::from(","),
                error_msg: String::new(),
                line: 0,
                start: 7,
                end: 8,
            },
            Token {
                token_type: TokenType::ConstChar,
                lexeme: String::from("\'3\'"),
                error_msg: String::new(),
                line: 0,
                start: 9,
                end: 12,
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: String::from(","),
                error_msg: String::new(),
                line: 0,
                start: 12,
                end: 13,
            },
            Token {
                token_type: TokenType::Instr,
                lexeme: String::from("true"),
                error_msg: String::new(),
                line: 0,
                start: 14,
                end: 18,
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: String::from(","),
                error_msg: String::new(),
                line: 0,
                start: 18,
                end: 19,
            },
            Token {
                token_type: TokenType::Instr,
                lexeme: String::from("false"),
                error_msg: String::new(),
                line: 0,
                start: 20,
                end: 25,
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: String::from(","),
                error_msg: String::new(),
                line: 0,
                start: 25,
                end: 26,
            },
            Token {
                token_type: TokenType::LeftSquare,
                lexeme: String::from("["),
                error_msg: String::new(),
                line: 0,
                start: 27,
                end: 28,
            },
            Token {
                token_type: TokenType::RightSquare,
                lexeme: String::from("]"),
                error_msg: String::new(),
                line: 0,
                start: 28,
                end: 29,
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: String::from(","),
                error_msg: String::new(),
                line: 0,
                start: 29,
                end: 30,
            },
            Token {
                token_type: TokenType::LeftCurly,
                lexeme: String::from("{"),
                error_msg: String::new(),
                line: 0,
                start: 31,
                end: 32,
            },
            Token {
                token_type: TokenType::RightCurly,
                lexeme: String::from("}"),
                error_msg: String::new(),
                line: 0,
                start: 32,
                end: 33,
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: String::from(","),
                error_msg: String::new(),
                line: 0,
                start: 33,
                end: 34,
            },
            Token {
                token_type: TokenType::ConstString,
                lexeme: String::from("\"\""),
                error_msg: String::new(),
                line: 0,
                start: 35,
                end: 37,
            },
            Token {
                token_type: TokenType::RightSquare,
                lexeme: String::from("]"),
                error_msg: String::new(),
                line: 0,
                start: 37,
                end: 38,
            },
        ];

        let input_str = String::from("[1, 2.2, '3', true, false, [], {}, \"\"]");
        assert_eq!(expected, parse_code(&input_str));
    }

    #[test]
    fn test_lex_hello_world() {
        let expected = vec![
            Token {
                token_type: TokenType::ConstString,
                lexeme: String::from("\"Hello World!\""),
                error_msg: String::new(),
                line: 0,
                start: 0,
                end: 15,
            },
            Token {
                token_type: TokenType::Tilde,
                lexeme: String::from("~"),
                error_msg: String::new(),
                line: 0,
                start: 15,
                end: 16,
            },
            Token {
                token_type: TokenType::Instr,
                lexeme: String::from("ow"),
                error_msg: String::new(),
                line: 0,
                start: 17,
                end: 19,
            },
            Token {
                token_type: TokenType::Tilde,
                lexeme: String::from("~"),
                error_msg: String::new(),
                line: 0,
                start: 19,
                end: 20,
            },
        ];
        let hello_world = String::from("\"Hello World!\"~ ow~");
        println!("Testing tokenizing `{}`", hello_world);
        assert_eq!(expected, parse_code(&hello_world));
    }

    #[test]
    fn test_lex_comments() {
        let input_str = String::from("# This is a comment\n# This is another #comment\n");
        let expected: Vec<Token> = Vec::new();
        assert_eq!(expected, parse_code(&input_str));
    }

    #[test]
    fn test_lex_comments_and_code() {
        let input_str =
            String::from("# This is a comment\nthis is some code\n# This is another #comment\n");
        let expected = vec![
            Token {
                token_type: TokenType::Instr,
                lexeme: String::from("this"),
                error_msg: String::new(),
                line: 1,
                start: 19,
                end: 24,
            },
            Token {
                token_type: TokenType::Instr,
                lexeme: String::from("is"),
                error_msg: String::new(),
                line: 1,
                start: 25,
                end: 27,
            },
            Token {
                token_type: TokenType::Instr,
                lexeme: String::from("some"),
                error_msg: String::new(),
                line: 1,
                start: 28,
                end: 32,
            },
            Token {
                token_type: TokenType::Instr,
                lexeme: String::from("code"),
                error_msg: String::new(),
                line: 1,
                start: 33,
                end: 38,
            },
        ];
        assert_eq!(expected, parse_code(&input_str));
    }
}
