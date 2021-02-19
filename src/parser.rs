use crate::util;

type Token = util::Token;

pub type Code = Vec<Op>;

pub enum Op {
    Instr,
    Literal,
    Block,
}

pub enum Block {
    Left(Code),
    Right(Code),
}

pub enum Instr {
    Left(Instruction),
    Right(Instruction),
}

pub enum Instruction {
    Literal(Literal),
    Instruction(Token),
}

pub enum Literal {
    Int(Token),
    Float(Token),
    Bool(Token),
    Char(Token),
    String(Token),
    List(List),
}

type List = Vec<Literal>;

pub fn parse(code: Vec<Token>) -> Code {
    let mut tree = Code::new();

    tree
}
