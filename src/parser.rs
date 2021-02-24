use crate::util::{lex, par};

type Token = lex::Token;
type Tt = lex::TokenType;
type Lt = lex::LiteralType;

enum ParseState {
    Begin,
    InOp,
    InBlock,
    InList,
}

pub fn parse(tokens: Vec<Token>) -> par::Code {
    let mut nodes = par::Code::new();

    for tok in tokens {
        match &tok.token_type {
            Tt::BlockBegin => {}
            Tt::BlockEnd => {}
            Tt::ListBegin => {}
            Tt::ListSep => {}
            Tt::ListEnd => {}
            Tt::Instruction => {}
            Tt::Literal(lt) => match lt {
                Int => {}
                Float => {}
                Bool => {}
                Char => {}
                String => {}
            },
            Tt::Bang => {}
        }
    }
    nodes
}
