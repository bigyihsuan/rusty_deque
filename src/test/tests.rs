#[cfg(test)]
mod tests {
    use super::lex::*;
    use super::lex_token::*;
    mod test_lex {
        #[test]
        fn test_int() {
            assert_eq!(
                get_next_token(String::from("0"), 0, 0).0,
                lex_token::Token {
                    token_type: TokenType::INT,
                    lexeme: String::from("0"),
                    line: 0,
                    start: 0,
                    end: 3,
                }
            );
            assert_eq!(
                get_next_token(String::from("123"), 0, 0).0,
                lex_token::Token {
                    token_type: TokenType::INT,
                    lexeme: String::from("123"),
                    line: 0,
                    start: 0,
                    end: 3,
                }
            );
            assert_eq!(
                get_next_token(String::from("-123"), 0, 0).0,
                lex_token::Token {
                    token_type: TokenType::INT,
                    lexeme: String::from("-123"),
                    line: 0,
                    start: 0,
                    end: 3,
                }
            );
        }
    }
}
