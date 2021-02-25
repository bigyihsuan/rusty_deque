use crate::util::{lex, par};

type Token = lex::Token;
type Tt = lex::TokenType;
type Lt = lex::LiteralType;

enum ParseState {
    Begin,
    InOpLeft,
    InOpRight,
    InOp,
    InBlock,
    InList,
    End,
}

pub fn parse(tokens: Vec<Token>) -> par::Code {
    type Ps = ParseState;
    let mut nodes = par::Code::new();
    let mut state = Ps::Begin;
    let mut tok_iter = tokens.iter();
    loop {
        let mut nodeLeft = par::ExecLeft {
            left: par::ExecT::None,
            right: None,
        };
        let mut nodeRight = par::ExecRight {
            left: None,
            right: par::ExecT::None,
        };

        let def = Token {
            token_type: Tt::End,
            string: "".to_owned(),
            index: usize::MAX,
        };
        let tok = tok_iter.next().unwrap_or(&def);

        if let Ps::End = state {
            break;
        }
    }
    nodes
}
