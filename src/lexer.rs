pub mod lex_token {

    // The type of a token.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TokenType {
        // SINGLE CHARACTERS
        Bang,
        Tilde,
        Comma,
        LeftCurly,
        RightCurly,
        LeftSquare,
        RightSquare,
        // MULTI CHARACTERS
        ConstInt,
        ConstFloat,
        ConstChar,
        ConstString,
        Instr,
        // Others
        Error,
        End,
    }

    // A lexical token.
    #[derive(Debug, Clone, PartialEq)]
    pub struct Token {
        pub token_type: TokenType,
        pub lexeme: String,
        pub error_msg: String,
        pub start: usize, // Start of the token in the source
        pub end: usize,   // end of the token in the source
        pub line: usize,  // line number of the token in the source
    }
}

pub mod lex {

    use super::lex_token::*;

    #[derive(Debug)]
    enum LexerState {
        Start,
        InInt,
        InFloat,
        InChar,
        InString,
        InInstrOrInt,
        InInstr,
    }

    // Gets the next token from an input string.
    // Returns the next token, as well as the index of the last character read, and the current line number.
    pub fn get_next_token(input: String, start: usize, line: usize) -> (Token, usize, usize) {
        let mut state = LexerState::Start;
        let mut lexeme = String::new();
        let mut error_msg = String::new();
        let mut i = start;
        let mut line = line;
        let mut token_type = TokenType::End;

        loop {
            println!(
                "{} {} '{}' {:?}",
                i,
                match input.chars().nth(i) {
                    Some(c) => c.to_string(),
                    None => "BLANK".to_string(),
                },
                lexeme,
                state
            );
            match state {
                LexerState::Start => {
                    // single-character tokens
                    match input.chars().nth(i) {
                        Some(' ') | Some('\t') => {
                            i += 1;
                        }
                        Some('\n') => {
                            line += 1;
                            i += 1;
                        }
                        Some('{') => {
                            token_type = TokenType::LeftCurly;
                            i += 1;
                            break;
                        }
                        Some('}') => {
                            token_type = TokenType::RightCurly;
                            i += 1;
                            break;
                        }
                        Some('[') => {
                            token_type = TokenType::LeftSquare;
                            i += 1;
                            break;
                        }
                        Some(']') => {
                            token_type = TokenType::RightSquare;
                            i += 1;
                            break;
                        }
                        Some(',') => {
                            token_type = TokenType::Comma;
                            i += 1;
                            break;
                        }
                        Some('!') => {
                            token_type = TokenType::Bang;
                            i += 1;
                            break;
                        }
                        Some('~') => {
                            token_type = TokenType::Tilde;
                            i += 1;
                            break;
                        }
                        Some('0'..='9') => {
                            state = LexerState::InInt;
                            i += 1;
                            lexeme.push(input.chars().nth(i - 1).unwrap());
                            token_type = TokenType::ConstInt;
                        }
                        Some('\'') => {
                            state = LexerState::InChar;
                            i += 1;
                            lexeme.push('\'');
                            token_type = TokenType::ConstChar;
                        }
                        Some('"') => {
                            state = LexerState::InString;
                            i += 1;
                            lexeme.push('"');
                            token_type = TokenType::ConstString;
                        }
                        Some('-') => {
                            state = LexerState::InInstrOrInt;
                            i += 1;
                            lexeme.push('-');
                        }
                        None => {
                            token_type = TokenType::End;
                            break;
                        }
                        _ => {
                            state = LexerState::InInstr;
                            i += 1;
                            lexeme.push(input.chars().nth(i - 1).unwrap());
                        }
                    }
                }
                LexerState::InInstrOrInt => match input.chars().nth(i) {
                    Some('0'..='9') => {
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                        state = LexerState::InInt;
                    }
                    _ => {
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                        state = LexerState::InInstr;
                    }
                },
                LexerState::InInt => match input.chars().nth(i) {
                    Some('0'..='9') => {
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                    }
                    Some('.') => {
                        state = LexerState::InFloat;
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                        token_type = TokenType::ConstFloat;
                    }
                    _ => {
                        token_type = TokenType::ConstInt;
                        break;
                    }
                },
                LexerState::InFloat => {
                    match input.chars().nth(i) {
                        Some('0'..='9') => {
                            i += 1;
                            lexeme.push(input.chars().nth(i - 1).unwrap());
                        }
                        Some('.') => {
                            error_msg = "Invalid float literal: Floats cannot contain multiple decimal points".to_string();
                            token_type = TokenType::Error;
                            break;
                        }
                        _ => {
                            if lexeme.chars().last().unwrap() == '.' {
                                error_msg = format!(
                                    "Invalid float literal: missing decimal portion in {}",
                                    lexeme
                                );
                                token_type = TokenType::Error;
                            }
                            break;
                        }
                    }
                }
                LexerState::InChar => match input.chars().nth(i) {
                    Some('\'') => {
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                        if lexeme.len() > 3 && lexeme.chars().nth(1).unwrap() != '\\' {
                            token_type = TokenType::Error;
                            error_msg =
                                "Invalid char literal: Character constant too long".to_string();
                        } else {
                            token_type = TokenType::ConstChar;
                        }
                        break;
                    }
                    Some(_) => {
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                    }
                    None => {
                        token_type = TokenType::Error;
                        error_msg =
                            "Invalid char literal: Unterminated character constant".to_string();
                        break;
                    }
                },
                LexerState::InString => match input.chars().nth(i) {
                    Some('"') => {
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                        token_type = TokenType::ConstString;
                        break;
                    }
                    None => {
                        // lexeme.push(input.chars().nth(i - 1).unwrap());
                        token_type = TokenType::Error;
                        error_msg =
                            "Invalid string literal: Unterminated string constant".to_string();
                        break;
                    }
                    _ => {
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                    }
                },
                LexerState::InInstr => match input.chars().nth(i) {
                    Some(' ') | Some('\t') => {
                        i += 1;
                        break;
                    }
                    Some(_) => {
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                    }
                    None => {
                        token_type = TokenType::Instr;
                        break;
                    }
                },
            }
        }

        (
            Token {
                token_type: token_type,
                lexeme: lexeme,
                error_msg: error_msg,
                start: start,
                end: i,
                line: line,
            },
            i,
            line,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lex::*;
    use crate::lexer::lex_token::*;

    #[test]
    fn test_const_int() {
        println!("Testing tokenizing ConstInt {}", 0);
        assert_eq!(
            get_next_token(String::from("0"), 0, 0).0,
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
            get_next_token(String::from("123"), 0, 0).0,
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
            get_next_token(String::from("-123"), 0, 0).0,
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
    fn test_const_float() {
        println!("Testing tokenizing ConstFloat {}", 0.0);
        assert_eq!(
            get_next_token(String::from("0.0"), 0, 0).0,
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
            get_next_token(String::from("123.123"), 0, 0).0,
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
            get_next_token(String::from("-123.123"), 0, 0).0,
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
            get_next_token(String::from("0.123"), 0, 0).0,
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
            get_next_token(String::from("0.123.123"), 0, 0).0,
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
    }

    #[test]
    fn test_const_char() {
        println!("Testing tokenizing ConstChar {}", 'a');
        assert_eq!(
            get_next_token(String::from("'a'"), 0, 0).0,
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
            get_next_token(String::from("'\\n'"), 0, 0).0,
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
            get_next_token(String::from("'a"), 0, 0).0,
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
            get_next_token(String::from("'ab'"), 0, 0).0,
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
    fn test_const_string() {
        println!("Testing tokenizing ConstString \"{}\"", "a");
        assert_eq!(
            get_next_token(String::from("\"a\""), 0, 0).0,
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
            get_next_token(String::from("\"abc\""), 0, 0).0,
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
            get_next_token(String::from("\"abcdef\""), 0, 0).0,
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
            get_next_token(String::from("\"abcd efghijklmnopqrst uvwxyz\""), 0, 0).0,
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
                String::from("\"abcdefghijklm\nopqrstuvwxyz\n0123456789\""),
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
            get_next_token(String::from("\"this is a test"), 0, 0).0,
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
}
