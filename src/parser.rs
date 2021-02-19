use crate::util;

type Token = util::Token;

pub type Code = Vec<Instr>;

#[derive(Debug)]
pub enum Instr {
    Left(Op),
    Right(Op),
}
#[derive(Debug)]
pub enum Op {
    Literal(Literal),
    Instruction(Token),
    Block(Code),
}
#[derive(Debug)]
pub enum Literal {
    Int(Token),
    Float(Token),
    Bool(Token),
    Char(Token),
    String(Token),
    List(List),
}

type List = Vec<Literal>;

pub fn parse(tokens: Vec<Token>) -> Code {
    let mut tree = Code::new();

    let mut i = 0;
    while i < tokens.len() {
        let tok = tokens.get(i).expect("outside of token bounds");

        // match tok.token_type {

        // }
        i += 1;
    }

    tree
}
