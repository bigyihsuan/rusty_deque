pub mod lex_token {
    use std::fmt::{Debug, Display, Error, Formatter};

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
    #[derive(Clone, PartialEq)]
    pub struct Token {
        pub token_type: TokenType,
        pub lexeme: String,
        pub error_msg: String,
        pub start: usize, // Start of the token in the source
        pub end: usize,   // end of the token in the source
        pub line: usize,  // line number of the token in the source
    }

    impl Debug for Token {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            write!(
                f,
                "{:?}({:?}, {:?}, {:?}, {:?}, {:?})",
                self.token_type, self.lexeme, self.error_msg, self.start, self.end, self.line
            )
        }
    }

    impl Display for Token {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            write!(
                f,
                "{:?}({:?}, {:?}, {:?}, {:?}, {:?})",
                self.token_type, self.lexeme, self.error_msg, self.start, self.end, self.line
            )
        }
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
        InComment,
    }

    // parses an entire code string into a vector of tokens
    // calls get_next_token() to get the next token
    pub fn parse_code(input: &String) -> Vec<Token> {
        let mut start: usize = 0;
        let mut line: usize = 0;
        let mut tokens = Vec::new();

        while start < input.len() {
            let (token, s, l) = get_next_token(&input, start, line);
            tokens.push(token);
            start = s;
            line = l;
        }
        if tokens.last().unwrap().token_type == TokenType::End {
            tokens.pop();
        }
        tokens
    }

    // Gets the next token from an input string.
    // Returns the next token, as well as the index of the last character read, and the current line number.
    pub fn get_next_token(input: &String, start: usize, line: usize) -> (Token, usize, usize) {
        let mut state = LexerState::Start;
        let mut lexeme = String::new();
        let mut error_msg = String::new();
        let mut s = start;
        let mut i = start;
        let mut line = line;
        let mut token_type = TokenType::End;

        loop {
            // println!(
            //     "{} {} '{}' {:?}",
            //     i,
            //     match input.chars().nth(i) {
            //         Some(c) => c.to_string(),
            //         None => "BLANK".to_string(),
            //     },
            //     lexeme,
            //     state
            // );
            match state {
                LexerState::Start => {
                    // single-character tokens
                    match input.chars().nth(i) {
                        Some(' ') | Some('\t') => {
                            s += 1;
                            i += 1;
                        }
                        Some('\n') => {
                            line += 1;
                            s += 1;
                            i += 1;
                        }
                        Some('{') => {
                            token_type = TokenType::LeftCurly;
                            i += 1;
                            lexeme.push(input.chars().nth(i - 1).unwrap());
                            break;
                        }
                        Some('}') => {
                            token_type = TokenType::RightCurly;
                            i += 1;
                            lexeme.push(input.chars().nth(i - 1).unwrap());
                            break;
                        }
                        Some('[') => {
                            token_type = TokenType::LeftSquare;
                            i += 1;
                            lexeme.push(input.chars().nth(i - 1).unwrap());
                            break;
                        }
                        Some(']') => {
                            token_type = TokenType::RightSquare;
                            i += 1;
                            lexeme.push(input.chars().nth(i - 1).unwrap());
                            break;
                        }
                        Some(',') => {
                            token_type = TokenType::Comma;
                            i += 1;
                            lexeme.push(input.chars().nth(i - 1).unwrap());
                            break;
                        }
                        Some('!') => {
                            token_type = TokenType::Bang;
                            i += 1;
                            lexeme.push(input.chars().nth(i - 1).unwrap());
                            break;
                        }
                        Some('~') => {
                            token_type = TokenType::Tilde;
                            i += 1;
                            lexeme.push(input.chars().nth(i - 1).unwrap());
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
                        Some('#') => {
                            state = LexerState::InComment;
                            i += 1;
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
                        // i += 1;
                        // lexeme.push(input.chars().nth(i - 1).unwrap());
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
                LexerState::InFloat => match input.chars().nth(i) {
                    Some('0'..='9') => {
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                    }
                    Some('.') => {
                        error_msg =
                            "Invalid float literal: Floats cannot contain multiple decimal points"
                                .to_string();
                        token_type = TokenType::Error;
                        break;
                    }
                    _ => {
                        if lexeme.chars().last().unwrap() == '.' {
                            error_msg =
                                "Invalid float literal: missing decimal portion".to_string();
                            token_type = TokenType::Error;
                        }
                        break;
                    }
                },
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
                    Some(' ') | Some('\t') | Some(',') | Some(']') | Some('}') | Some('~')
                    | Some('!') => {
                        token_type = TokenType::Instr;
                        break;
                    }
                    Some('\n') => {
                        i += 1;
                        token_type = TokenType::Instr;
                        break;
                    }
                    Some(_) => {
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                    }
                    _ => {
                        token_type = TokenType::Instr;
                        break;
                    }
                },
                LexerState::InComment => match input.chars().nth(i) {
                    Some('\n') => {
                        i += 1;
                        s += 1;
                        line += 1;
                        state = LexerState::Start;
                        lexeme = String::new()
                    }
                    Some(_) => {
                        i += 1;
                        s += 1;
                    }
                    None => {
                        token_type = TokenType::End;
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
                start: s,
                end: i,
                line: line,
            },
            i,
            line,
        )
    }
}
