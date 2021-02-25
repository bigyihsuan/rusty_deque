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
        End,
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

    trait Visitable {
        fn visit(&self) {}
    }

    trait Evaluatable {
        fn eval(&self) {
            unimplemented!();
        }
    }

    pub enum NodeT {
        ExecT(ExecT),
        OpT(OpT),
        LitT(LitT),
    }

    #[derive(Debug)]
    pub enum ExecT {
        Left,
        Right,
        None,
    }

    #[derive(Debug)]
    pub struct ExecLeft {
        pub left: ExecT,
        pub right: Option<Box<Op>>,
    }

    impl Visitable for ExecLeft {
        fn visit(&self) {
            print!("Exec {{ ! ");
            match self.right.as_ref() {
                Some(_) => {
                    self.right.as_ref().expect("No op for ExecLeft").visit();
                }
                None => {}
            }
            println!(" }}");
        }
    }

    impl ExecLeft {
        fn new(left: ExecT, right: Option<Box<Op>>) -> ExecLeft {
            ExecLeft { left, right }
        }
    }

    #[derive(Debug)]
    pub struct ExecRight {
        pub left: Option<Box<Op>>,
        pub right: ExecT,
    }

    impl Visitable for ExecRight {
        fn visit(&self) {
            print!("Exec {{ ! ");
            match self.left.as_ref() {
                Some(_) => {
                    self.left.as_ref().expect("No op for ExecRight").visit();
                }
                None => {}
            }
            println!(" }}");
        }
    }

    impl ExecRight {
        fn new(left: Option<Box<Op>>, right: ExecT) -> ExecRight {
            ExecRight { left, right }
        }
    }

    #[derive(Debug)]
    pub enum OpT {
        Literal(LitT),
        Instruction(Instr),
        None,
    }

    #[derive(Debug)]
    pub struct Op {
        pub left: OpT,
        pub right: Option<Box<Op>>,
    }

    impl Visitable for Op {
        fn visit(&self) {
            println!("{:?}", self.left);
            match self.right.as_ref() {
                Some(_) => {
                    self.right.as_ref().expect("").visit();
                }
                None => {}
            }
        }
    }

    impl Op {
        fn new(left: OpT, right: Option<Box<Op>>) -> Op {
            Op { left, right }
        }
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
        None,
    }
    #[derive(Debug)]
    pub struct Lit(LitT);

    impl Lit {
        fn new(value: LitT) -> Lit {
            Lit(value)
        }
    }

    impl Visitable for Lit {
        fn visit(&self) {
            println!("{:?}", self.0);
        }
    }

    pub type Code = Vec<ExecT>;

    pub type Instr = Tok;
    pub type DoubQuot = Tok;
    pub type CurlOpen = Tok;
    pub type CurlClose = Tok;
}
