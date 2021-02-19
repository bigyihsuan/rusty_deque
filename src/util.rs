#[derive(Debug)]
pub enum TokenType {
    BlockBegin,
    BlockEnd,
    ListBegin,
    ListSep,
    ListEnd,
    Instruction,
    Literal(LiteralType),
    Bang,
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
