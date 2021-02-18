#[derive(Debug)]
pub enum TokenType {
    Comment,
    BlockBegin,
    BlockEnd,
    ListBegin,
    ListEnd,
    Instruction,
    Literal(LiteralType),
    Bang,
    End,
}

#[derive(Debug)]
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
}

impl Token {
    pub fn new(token_type: TokenType, string: String, index: usize) -> Token {
        Token {
            token_type,
            string,
            index,
        }
    }
}

#[derive(Debug)]
pub enum LexerState {
    Begin,
    InComment,
    InInstructionOrBool,
    InInstruction,
    InString,
    InChar,
    InNumber,
    InInt,
    InFloat,
}

/// Converts code string into a Vec<Token>.
///
/// # Arguments
/// * `code` - A string containg source code
/// # Returns
/// * A Vec<Token> containing all tokens in the code string.
pub fn tokenize(code: String) -> Vec<Token> {
    type Ls = LexerState;
    type Tt = TokenType;
    type Lt = LiteralType;
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
                // ] => list end
                // " => string literal start
                // ' => char literal start
                // digit => number literal
                // sumbol => instruction
                // non-digit => instruction or bool
                // whitespace => ignore, pop out
                token_string.push(c);
                match c {
                    '#' => {
                        current_state = Ls::InComment;
                        token_string = String::new();
                    }
                    '!' => {
                        tokens.push(Token::new(Tt::Bang, token_string, i));
                        token_string = String::new();
                    }
                    '{' => {
                        tokens.push(Token::new(Tt::BlockBegin, token_string, i));
                        token_string = String::new();
                    }
                    '}' => {
                        tokens.push(Token::new(Tt::BlockEnd, token_string, i));
                        token_string = String::new();
                    }
                    '[' => {
                        tokens.push(Token::new(Tt::ListBegin, token_string, i));
                        token_string = String::new();
                    }
                    ']' => {
                        tokens.push(Token::new(Tt::ListEnd, token_string, i));
                        token_string = String::new();
                    }
                    '"' => {
                        current_state = Ls::InString;
                    }
                    '0'..='9' => {
                        current_state = Ls::InNumber;
                    }
                    '$' | ':' | '@' | '^' | '+' | '-' | '*' | '/' | '&' | '|' | '>' | '<' => {
                        current_state = Ls::InInstruction;
                    }
                    'A'..='Z' | 'a'..='z' => {
                        current_state = Ls::InInstructionOrBool;
                    }
                    c if c.is_whitespace() => {
                        tokens.pop();
                    }
                    _ => {}
                }
            }
            Ls::InComment => {
                // possible characters:
                // '\n' => go back to beginning
                // else => continue
                if let '\n' = c {
                    current_state = Ls::Begin;
                }
            }
            Ls::InInstructionOrBool => {
                let tok_str = &token_string[..];
                match tok_str {
                    "true" | "false" => {
                        tokens.push(Token::new(Tt::Literal(Lt::Bool), token_string, i));
                        token_string = String::new();
                        current_state = Ls::Begin;
                    }
                    tok_str if tok_str.chars().last().expect("").is_whitespace() => {
                        current_state = Ls::InInstruction;
                    }
                    &_ => {
                        token_string.push(c);
                    }
                }
            }
            Ls::InInstruction => match c {
                c if c.is_whitespace() => {
                    tokens.push(Token::new(Tt::Instruction, token_string, i));
                    token_string = String::new();
                    current_state = Ls::Begin;
                }
                _ => {
                    token_string.push(c);
                }
            },
            Ls::InString => match c {
                '"' => {
                    tokens.push(Token::new(Tt::Literal(Lt::String), token_string, i));
                    token_string = String::new();
                    current_state = Ls::Begin;
                }
                _ => {
                    token_string.push(c);
                }
            },
            Ls::InChar => match c {
                '\'' => {
                    tokens.push(Token::new(Tt::Literal(Lt::Char), token_string, i));
                    token_string = String::new();
                    current_state = Ls::Begin;
                }
                _ => {
                    token_string.push(c);
                }
            },
            Ls::InNumber => {
                unimplemented!()
            }
            Ls::InInt => {
                unimplemented!()
            }
            Ls::InFloat => {
                unimplemented!()
            }
        }
        i += 1
    }
    tokens
}
