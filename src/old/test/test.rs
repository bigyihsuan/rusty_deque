mod evaluator;
mod lexer;
mod parser;

fn main() {
    assert_eq!(
        lexer::lex::tokenize(String::from("a!")),
        vec![
            Token {
                token_type: TokenType::Instruction,
                string: String::from("a"),
                index: 0,
                tok_index: 0
            },
            Token {
                token_type: TokenType::Bang,
                string: String::from("!"),
                index: 1,
                tok_index: 1
            }
        ]
    );
}
