pub mod lex {
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
}

pub mod par {
    type Tok = crate::util::lex::Token;
    #[derive(Debug)]
    pub enum ExecT {
        Left,
        Right,
    }
    #[derive(Debug)]
    pub enum OpT {
        Literal(LitT),
        Instruction(Instr),
    }
    #[derive(Debug)]
    pub enum LitT {
        Int(i64),
        Float(f64),
        Bool(bool),
        Char(char),
        String(DoubQuot, String, DoubQuot),
        List(Vec<LitT>),
        Block(CurlOpen, Box<Code>, CurlClose),
    }
    pub type Code = Vec<Exec>;

    pub type Instr = Tok;
    pub type DoubQuot = Tok;
    pub type CurlOpen = Tok;
    pub type CurlClose = Tok;

    trait Visitable {
        fn visit(&self) {}
    }

    trait Evaluatable {
        fn eval(&self) {
            unimplemented!();
        }
    }

    #[derive(Debug)]
    pub struct Exec {
        side: ExecT,
        op: Op,
    }

    impl Visitable for Exec {
        fn visit(&self) {
            match &self.side {
                ExecT::Left => {
                    print!("Exec {{ ! ");
                    self.op.visit();
                    println!(" }}");
                }
                ExecT::Right => {
                    print!("Exec {{ ");
                    self.op.visit();
                    println!("! }}");
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct Op {
        left: OpT,
        right: Box<Op>,
    }

    impl Visitable for Op {
        fn visit(&self) {
            println!("{:?}", self.left);
            (*self.right).visit();
        }
    }

    #[derive(Debug)]
    pub struct Lit {
        literal: LitT,
    }

    impl Visitable for Lit {
        fn visit(&self) {
            println!("{:?}", self.literal);
        }
    }
}
