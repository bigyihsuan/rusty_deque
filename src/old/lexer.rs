pub mod tok {
    #[derive(Debug, PartialEq)]
    pub enum TokenType {
        BlockBegin,
        BlockEnd,
        ListBegin,
        ListSep,
        ListEnd,
        Instruction,
        Literal(LiteralType),
        Bang,
        Tilde,
        End,
    }

    #[derive(Debug, PartialEq)]
    pub enum LiteralType {
        Int,
        Float,
        Bool,
        Char,
        String,
    }

    #[derive(Debug)]
    pub struct Token {
        pub token_type: TokenType,
        pub string: String,
        pub index: usize,
        pub tok_index: usize,
    }
    impl Token {
        pub fn new(token_type: TokenType, string: String, index: usize, tok_index: usize) -> Token {
            Token {
                token_type,
                string,
                index,
                tok_index,
            }
        }
    }
    impl Default for Token {
        fn default() -> Token {
            Token {
                token_type: TokenType::End,
                string: "".to_string(),
                index: usize::MAX,
                tok_index: usize::MAX,
            }
        }
    }
}

pub mod lex {

    use crate::lexer::tok;

    #[derive(Debug)]
    pub enum LexerState {
        Begin,
        InComment,
        InInstructionOrBool,
        InInstructionOrNumber,
        InInstruction,
        InString,
        InChar,
        InNumber,
        InFloat,
    }

    type Token = tok::Token;
    type Tt = tok::TokenType;
    type Lt = tok::LiteralType;

    /// Converts code string into a Vec<Token>.
    ///
    /// # Arguments
    /// * `code` - A string containg source code
    /// # Returns
    /// * A Vec<Token> containing all tokens in the code string.
    pub fn tokenize(code: String) -> Vec<Token> {
        let code_chars: Vec<char> = code.chars().collect();
        let mut tokens = Vec::new();
        let mut current_state = LexerState::Begin;

        let mut i: usize = 0;
        let mut ti: usize = 0;
        let mut token_string = String::new();
        while i < code_chars.len() {
            let c = *code_chars.get(i).expect("Outside of code string range") as char;
            match current_state {
                LexerState::Begin => {
                    // possible characters encountered:
                    // # => comment
                    // ! => bang
                    // ~ => tilde
                    // { => block start
                    // } => block end
                    // [ => list start
                    // , => list separator
                    // ] => list end
                    // " => string literal start
                    // ' => char literal start
                    // - => instruction or number literal
                    // digit => number literal
                    // sumbol => instruction
                    // non-digit => instruction or bool
                    // whitespace => ignore, pop out
                    match c {
                        '#' => {
                            current_state = LexerState::InComment;
                            token_string = String::new();
                        }
                        '!' => {
                            token_string.push(c);
                            tokens.push(Token::new(Tt::Bang, token_string, i, ti));
                            token_string = String::new();
                            i += 1;
                            ti += 1;
                        }
                        '~' => {
                            token_string.push(c);
                            tokens.push(Token::new(Tt::Tilde, token_string, i, ti));
                            token_string = String::new();
                            i += 1;
                            ti += 1;
                        }
                        '{' => {
                            token_string.push(c);
                            tokens.push(Token::new(Tt::BlockBegin, token_string, i, ti));
                            token_string = String::new();
                            i += 1;
                            ti += 1;
                        }
                        '}' => {
                            token_string.push(c);
                            tokens.push(Token::new(Tt::BlockEnd, token_string, i, ti));
                            token_string = String::new();
                            i += 1;
                            ti += 1;
                        }
                        '[' => {
                            token_string.push(c);
                            tokens.push(Token::new(Tt::ListBegin, token_string, i, ti));
                            token_string = String::new();
                            i += 1;
                            ti += 1;
                        }
                        ',' => {
                            token_string.push(c);
                            tokens.push(Token::new(Tt::ListSep, token_string, i, ti));
                            token_string = String::new();
                            i += 1;
                            ti += 1;
                        }
                        ']' => {
                            token_string.push(c);
                            tokens.push(Token::new(Tt::ListEnd, token_string, i, ti));
                            token_string = String::new();
                            i += 1;
                            ti += 1;
                        }
                        '"' => {
                            // token_string.push(c);
                            current_state = LexerState::InString;
                            i += 1;
                        }
                        '\'' => {
                            // token_string.push(c);
                            current_state = LexerState::InChar;
                            i += 1;
                        }
                        '0'..='9' => {
                            token_string.push(c);
                            current_state = LexerState::InNumber;
                            i += 1;
                        }
                        '$' | ':' | '@' | '^' | '+' | '*' | '/' | '&' | '|' | '>' | '<' => {
                            token_string.push(c);
                            current_state = LexerState::InInstruction;
                            i += 1;
                        }
                        '-' => {
                            token_string.push(c);
                            current_state = LexerState::InInstructionOrNumber;
                            i += 1;
                        }
                        'A'..='Z' | 'a'..='z' => {
                            token_string.push(c);
                            current_state = LexerState::InInstructionOrBool;
                            i += 1;
                        }
                        _ => i += 1,
                    }
                }
                LexerState::InComment => {
                    // possible characters:
                    // '\n' => go back to beginning
                    // else => continue
                    if let '\n' = c {
                        current_state = LexerState::Begin;
                    } else {
                        i += 1;
                    }
                }
                LexerState::InInstructionOrBool => {
                    let tok_str = &token_string[..];
                    match tok_str {
                        "true" | "false" => {
                            tokens.push(Token::new(Tt::Literal(Lt::Bool), token_string, i, ti));
                            token_string = String::new();
                            current_state = LexerState::Begin;
                            i += 1;
                            ti += 1;
                        }
                        &_ => match c {
                            c if c.is_whitespace() => {
                                current_state = LexerState::InInstruction;
                            }
                            '!' | '~' | '{' | '}' | '[' | ']' | '"' => {
                                current_state = LexerState::InInstruction;
                            }
                            _ => {
                                token_string.push(c);
                                i += 1;
                            }
                        },
                    }
                }
                LexerState::InInstructionOrNumber => match c {
                    '0'..='9' => {
                        current_state = LexerState::InNumber;
                    }
                    _ => {
                        current_state = LexerState::InInstruction;
                    }
                },
                LexerState::InInstruction => match c {
                    c if c.is_whitespace() => {
                        tokens.push(Token::new(Tt::Instruction, token_string, i, ti));
                        token_string = String::new();
                        ti += 1;
                        current_state = LexerState::Begin;
                    }
                    '!' | '~' | '{' | '}' | '[' | ']' | '"' => {
                        tokens.push(Token::new(Tt::Instruction, token_string, i, ti));
                        token_string = String::new();
                        current_state = LexerState::Begin;
                        ti += 1;
                    }
                    _ => {
                        token_string.push(c);
                        i += 1;
                    }
                },
                LexerState::InString => match c {
                    '"' => {
                        tokens.push(Token::new(Tt::Literal(Lt::String), token_string, i, ti));
                        token_string = String::new();
                        current_state = LexerState::Begin;
                        i += 1;
                        ti += 1;
                    }
                    _ => {
                        token_string.push(c);
                        i += 1;
                    }
                },
                LexerState::InChar => match c {
                    '\'' => {
                        tokens.push(Token::new(Tt::Literal(Lt::Char), token_string, i, ti));
                        token_string = String::new();
                        current_state = LexerState::Begin;
                        i += 1;
                        ti += 1;
                    }
                    _ => {
                        token_string.push(c);
                        i += 1;
                    }
                },
                LexerState::InNumber => match c {
                    c if c.is_whitespace() => {
                        tokens.push(Token::new(Tt::Literal(Lt::Int), token_string, i, ti));
                        token_string = String::new();
                        current_state = LexerState::Begin;
                        ti += 1;
                    }
                    '!' | '~' | '{' | '}' | '[' | ']' | '"' | ',' => {
                        tokens.push(Token::new(Tt::Literal(Lt::Int), token_string, i, ti));
                        token_string = String::new();
                        current_state = LexerState::Begin;
                        ti += 1;
                    }
                    '.' => {
                        current_state = LexerState::InFloat;
                        token_string.push(c);
                        i += 1;
                    }
                    _ => {
                        token_string.push(c);
                        i += 1;
                    }
                },
                LexerState::InFloat => match c {
                    c if c.is_whitespace() => {
                        tokens.push(Token::new(Tt::Literal(Lt::Float), token_string, i, ti));
                        token_string = String::new();
                        current_state = LexerState::Begin;
                        ti += 1;
                    }
                    '!' | '~' | '{' | '}' | '[' | ']' | '"' | ',' => {
                        tokens.push(Token::new(Tt::Literal(Lt::Float), token_string, i, ti));
                        token_string = String::new();
                        current_state = LexerState::Begin;
                        ti += 1;
                    }
                    _ => {
                        token_string.push(c);
                        i += 1;
                    }
                },
            }
        }
        tokens.push(Token::new(Tt::End, "".to_string(), i, ti));
        tokens
    }
}
