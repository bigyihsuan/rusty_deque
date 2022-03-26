pub mod lex_token {

    // The type of a token.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TokenType {
        // SINGLE CHARACTERS
        BANG,
        TILDE,
        COMMA,
        LEFT_CURLY,
        RIGHT_CURLY,
        LEFT_SQUARE,
        RIGHT_SQUARE,
        SINGLE_QUOTE,
        DOUBLE_QUOTE,
        // MULTI-CHARACTERs
        TRUE,
        FALSE,
        CONST_INT,
        CONST_FLOAT,
        CONST_CHAR,
        CONST_STRING,
        INSTR,
        // OTHERS
        ERROR,
        END,
    }

    // A lexical token.
    #[derive(Debug, Clone, PartialEq, Default)]
    pub struct Token {
        pub token_type: TokenType,
        pub lexeme: String,
        pub error_msg: String,
        pub start: usize, // start of the token in the source
        pub end: usize,   // end of the token in the source
        pub line: usize,  // line number of the token in the source
    }
}

pub mod lex {
    use super::lex_token::*;

    enum LexerState {
        START,
        IN_INT,
        IN_FLOAT,
        IN_STRING,
        ININSTR_OR_INT,
        ININSTR_OR_BOOL,
    }

    // Gets the next token from an input string.
    // Returns the next token, as well as the index of the last character read, and the current line number.
    fn get_next_token(input: String, start: usize, line: usize) -> (Token, usize, usize) {
        let mut state = LexerState::START;
        let mut lexeme = String::new();
        let mut error_msg = String::new();
        let mut i = start;
        let mut line = line;
        let mut token_type = TokenType::INSTR;

        loop {
            match state {
                LexerState::START => {
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
                            token_type = TokenType::LEFT_CURLY;
                            i += 1;
                            break;
                        }
                        Some('}') => {
                            token_type = TokenType::RIGHT_CURLY;
                            i += 1;
                            break;
                        }
                        Some('[') => {
                            token_type = TokenType::LEFT_SQUARE;
                            i += 1;
                            break;
                        }
                        Some(']') => {
                            token_type = TokenType::RIGHT_SQUARE;
                            i += 1;
                            break;
                        }
                        Some(',') => {
                            token_type = TokenType::COMMA;
                            i += 1;
                            break;
                        }
                        Some('!') => {
                            token_type = TokenType::BANG;
                            i += 1;
                            break;
                        }
                        Some('~') => {
                            token_type = TokenType::TILDE;
                            i += 1;
                            break;
                        }
                        Some('0'...'9') => {
                            state = LexerState::IN_INT;
                            i += 1;
                            lexeme.push(input.chars().nth(i - 1).unwrap());
                            token_type = TokenType::CONST_INT;
                        }
                        Some('\'') => {
                            state = LexerState::IN_CHAR;
                            i += 1;
                            lexeme.push('\'');
                            token_type = TokenType::CONST_CHAR;
                        }
                        Some('"') => {
                            state = LexerState::IN_STRING;
                            i += 1;
                            lexeme.push('"');
                            token_type = TokenType::CONST_STRING;
                        }
                        Some('-') => {
                            state = LexerState::ININSTR_OR_INT;
                            i += 1;
                            lexeme.push('-');
                        }
                        None => {
                            token_type = TokenType::END;
                            break;
                        }
                        _ => {
                            state = LexerState::ININSTR_OR_BOOL;
                            i += 1;
                            lexeme.push(input.chars().nth(i - 1).unwrap());
                        }
                    }
                }
                LexerState::ININSTR_OR_INT => match input.chars().nth(i) {
                    Some('0'...'9') => {
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                        i += 1;
                        lexer_state = LexerState::IN_INT;
                    }
                    _ => {
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                        i += 1;
                        lexer_state = LexerState::IN_INSTR;
                    }
                },
                LexerState::IN_INT => match input.chars().nth(i) {
                    Some('0'...'9') => {
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                    }
                    Some('.') => {
                        state = LexerState::IN_FLOAT;
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                        token_type = TokenType::CONST_FLOAT;
                    }
                    _ => {
                        break;
                    }
                },
                LexerState::IN_FLOAT => match input.chars().nth(i) {
                    Some('0'...'9') => {
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                    }
                    _ => {
                        if lexeme.chars().last().unwrap() == '.' {
                            error_msg = format!(
                                "Invalid float literal: missing decimal portion in {}",
                                lexeme
                            );
                            token_type = TokenType::ERROR;
                        }
                        break;
                    }
                },
                LexerState::IN_CHAR => match input.chars().nth(i) {
                    Some('\'') => {
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                        break;
                    }
                    Some(_) => {
                        if lexeme.len() > 2 {
                            break;
                        }
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                    }
                    None => {
                        token_type = TokenType::ERROR;
                        error_msg = "Unterminated character constant".to_string();
                        break;
                    }
                },
                LexerState::IN_STRING => match input.chars().nth(i) {
                    Some('"') => {
                        i += 1;
                        break;
                    }
                    None => {
                        token_type = TokenType::ERROR;
                        error_msg = "Unterminated string constant".to_string();
                        break;
                    }
                    _ => {
                        i += 1;
                        lexeme.push(input.chars().nth(i - 1).unwrap());
                    }
                },
            }
        }

        (
            Token {
                token_type: token_type,
                lexeme: lexeme,
                start: start,
                end: i,
                line: line,
            },
            i,
            line,
        )
    }
}
