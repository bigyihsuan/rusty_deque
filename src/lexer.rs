pub mod inner {
    pub fn hello() {
        println!("Hello!");
    }
}

enum TokenType {
    Comment,
    BlockBegin,
    BlockEnd,
    ListBegin,
    ListEnd,
    Instruction,
    Literal,
}

struct Token {
    pub token_type: TokenType,
    pub string: String,
    pub index: usize,
}

impl Token {
    fn new(token_type: TokenType, string: String, index: usize) -> Token {
        Token {
            token_type,
            string,
            index,
        }
    }
}

fn get_next_token(code: String) -> (String, Token) {
    unimplemented!()
}
