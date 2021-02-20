use crate::util;

type Token = util::Token;
type Tt = util::TokenType;
type Lt = util::LiteralType;

pub type CodeType = Vec<InstrType>;

#[derive(Debug)]
pub enum InstrType {
    Left(OpType),
    Right(OpType),
}

#[derive(Debug)]
pub enum OpType {
    Literal(LiteralType),
    Instruction(InstructionType),
}
#[derive(Debug)]
pub enum InstructionType {
    Instruction(Token),
}

#[derive(Debug)]
pub enum LiteralType {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(Vec<LiteralType>),
    List(Vec<LiteralType>),
    Block(CodeType),
}

pub struct Code {
    node_type: CodeType,
    children: Vec<Op>,
}

pub struct Instr {
    node_type: InstrType,
    op: Op,
}

pub struct Op {
    node_type: OpType,
    token: Token,
}

pub struct Instruction {
    node_type: InstructionType,
}

pub struct Literal {
    value: LiteralType,
}

pub struct Node {
    node_type: Option<InstrType>,
    token: Option<Token>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            node_type: None,
            token: None,
            left: None,
            right: None,
        }
    }
}

struct Ast {
    root: Option<Code>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast { root: None }
    }
}

pub fn parse(tokens: Vec<Token>) -> Ast {
    let mut tree = Ast { root: None };

    let mut i = 0;
    while i < tokens.len() {
        let tok = tokens.get(i).expect("outside of token bounds");
        match &tok.token_type {
            Tt::BlockBegin => {}
            Tt::BlockEnd => {}
            Tt::ListBegin => {}
            Tt::ListSep => {}
            Tt::ListEnd => {}
            Tt::Instruction => {}
            Tt::Literal(Lt::Int) => {}
            Tt::Literal(Lt::Float) => {}
            Tt::Literal(Lt::Bool) => {}
            Tt::Literal(Lt::Char) => {}
            Tt::Literal(Lt::String) => {}
            Tt::Bang => {}
        }
        i += 1;
    }

    tree
}
