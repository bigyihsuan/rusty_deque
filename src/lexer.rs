use crate::util;

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

type Token = util::Token;
type Tt = util::TokenType;
type Lt = util::LiteralType;

/// Converts code string into a Vec<Token>.
///
/// # Arguments
/// * `code` - A string containg source code
/// # Returns
/// * A Vec<Token> containing all tokens in the code string.
pub fn tokenize(code: String) -> Vec<Token> {
    type Ls = LexerState;
    let code_bytes = code.into_bytes();
    let mut tokens = Vec::new();
    let mut current_state = Ls::Begin;

    let mut i: usize = 0;
    let mut token_string = String::new();
    while i < code_bytes.len() {
        let c = *code_bytes.get(i).expect("Outside of code string range") as char;
        match current_state {
            Ls::Begin => {
                // possible characters encountered:
                // # => comment
                // ! => bang
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
                        current_state = Ls::InComment;
                        token_string = String::new();
                    }
                    '!' => {
                        token_string.push(c);
                        tokens.push(Token::new(Tt::Bang, token_string, i));
                        token_string = String::new();
                        i += 1;
                    }
                    '{' => {
                        token_string.push(c);
                        tokens.push(Token::new(Tt::BlockBegin, token_string, i));
                        token_string = String::new();
                        i += 1;
                    }
                    '}' => {
                        token_string.push(c);
                        tokens.push(Token::new(Tt::BlockEnd, token_string, i));
                        token_string = String::new();
                        i += 1;
                    }
                    '[' => {
                        token_string.push(c);
                        tokens.push(Token::new(Tt::ListBegin, token_string, i));
                        token_string = String::new();
                        i += 1;
                    }
                    ',' => {
                        token_string.push(c);
                        tokens.push(Token::new(Tt::ListSep, token_string, i));
                        token_string = String::new();
                        i += 1;
                    }
                    ']' => {
                        token_string.push(c);
                        tokens.push(Token::new(Tt::ListEnd, token_string, i));
                        token_string = String::new();
                        i += 1;
                    }
                    '"' => {
                        token_string.push(c);
                        current_state = Ls::InString;
                        i += 1;
                    }
                    '0'..='9' => {
                        token_string.push(c);
                        current_state = Ls::InNumber;
                        i += 1;
                    }
                    '$' | ':' | '@' | '^' | '+' | '*' | '/' | '&' | '|' | '>' | '<' => {
                        token_string.push(c);
                        current_state = Ls::InInstruction;
                        i += 1;
                    }
                    '-' => {
                        token_string.push(c);
                        current_state = Ls::InInstructionOrNumber;
                        i += 1;
                    }
                    'A'..='Z' | 'a'..='z' => {
                        token_string.push(c);
                        current_state = Ls::InInstructionOrBool;
                        i += 1;
                    }
                    _ => i += 1,
                }
            }
            Ls::InComment => {
                // possible characters:
                // '\n' => go back to beginning
                // else => continue
                if let '\n' = c {
                    current_state = Ls::Begin;
                } else {
                    i += 1;
                }
            }
            Ls::InInstructionOrBool => {
                let tok_str = &token_string[..];
                match tok_str {
                    "true" | "false" => {
                        tokens.push(Token::new(Tt::Literal(Lt::Bool), token_string, i));
                        token_string = String::new();
                        current_state = Ls::Begin;
                        i += 1;
                    }
                    &_ => match c {
                        c if c.is_whitespace() => {
                            current_state = Ls::InInstruction;
                        }
                        '!' | '{' | '}' | '[' | ']' | '"' => {
                            current_state = Ls::InInstruction;
                        }
                        _ => {
                            token_string.push(c);
                            i += 1;
                        }
                    },
                }
            }
            Ls::InInstructionOrNumber => match c {
                '0'..='9' => {
                    current_state = Ls::InNumber;
                }
                _ => {
                    current_state = Ls::InInstruction;
                }
            },
            Ls::InInstruction => match c {
                c if c.is_whitespace() => {
                    tokens.push(Token::new(Tt::Instruction, token_string, i));
                    token_string = String::new();
                    current_state = Ls::Begin;
                }
                '!' | '{' | '}' | '[' | ']' | '"' => {
                    tokens.push(Token::new(Tt::Instruction, token_string, i));
                    token_string = String::new();
                    current_state = Ls::Begin;
                }
                _ => {
                    token_string.push(c);
                    i += 1;
                }
            },
            Ls::InString => match c {
                '"' => {
                    tokens.push(Token::new(Tt::Literal(Lt::String), token_string, i));
                    token_string = String::new();
                    current_state = Ls::Begin;
                    i += 1;
                }
                _ => {
                    token_string.push(c);
                    i += 1;
                }
            },
            Ls::InChar => match c {
                '\'' => {
                    tokens.push(Token::new(Tt::Literal(Lt::Char), token_string, i));
                    token_string = String::new();
                    current_state = Ls::Begin;
                    i += 1;
                }
                _ => {
                    token_string.push(c);
                    i += 1;
                }
            },
            Ls::InNumber => match c {
                c if c.is_whitespace() => {
                    tokens.push(Token::new(Tt::Literal(Lt::Int), token_string, i));
                    token_string = String::new();
                    current_state = Ls::Begin;
                }
                '!' | '{' | '}' | '[' | ']' | '"' | ',' => {
                    tokens.push(Token::new(Tt::Literal(Lt::Int), token_string, i));
                    token_string = String::new();
                    current_state = Ls::Begin;
                }
                '.' => {
                    current_state = Ls::InFloat;
                    token_string.push(c);
                    i += 1;
                }
                _ => {
                    token_string.push(c);
                    i += 1;
                }
            },
            Ls::InFloat => match c {
                c if c.is_whitespace() => {
                    tokens.push(Token::new(Tt::Literal(Lt::Float), token_string, i));
                    token_string = String::new();
                    current_state = Ls::Begin;
                }
                '!' | '{' | '}' | '[' | ']' | '"' | ',' => {
                    tokens.push(Token::new(Tt::Literal(Lt::Float), token_string, i));
                    token_string = String::new();
                    current_state = Ls::Begin;
                }
                _ => {
                    token_string.push(c);
                    i += 1;
                }
            },
        }
    }
    tokens
}
