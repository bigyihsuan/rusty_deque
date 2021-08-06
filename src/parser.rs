pub mod ast {
    // leaves:
    // * Instruction
    // * all literals except Block, List
    pub type Code = Vec<Exec>;
    #[derive(Debug)]
    pub enum Exec {
        Left(Op),
        Right(Op),
    }
    #[derive(Debug)]
    pub enum Op {
        Literal(Literal),
        Instruction(Instruction),
    }
    #[derive(Debug)]
    pub struct Instruction {
        pub value: String,
    }
    #[derive(Debug)]
    pub enum Literal {
        Integer(i64),
        Float(f64),
        Boolean(bool),
        Character(char),
        List(List),   // not a leaf
        Block(Block), // not a leaf
    }

    pub type List = Vec<Box<Literal>>;
    pub type Block = Code;
}

pub mod visit {
    // https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html
    // https://github.com/rust-unofficial/patterns/discussions/236
    use super::ast::*;

    pub trait Visitor<T> {
        fn visit_code(&mut self, c: &Code) -> T;
        fn visit_exec(&mut self, e: &Exec) -> T;
        fn visit_op(&mut self, o: &Op) -> T;
        fn visit_instruction(&mut self, i: &Instruction) -> T;
        fn visit_literal(&mut self, l: &Literal) -> T;
    }

    pub struct TreePrinter {
        pub indent: usize,
    }
    impl TreePrinter {
        fn new() -> TreePrinter {
            TreePrinter { indent: 0 }
        }
    }

    impl Visitor<String> for TreePrinter {
        fn visit_code(&mut self, c: &Code) -> String {
            let mut s: String = String::from("( \n");
            self.indent += 1;
            for e in &*c {
                for _ in 0..self.indent {
                    s += "    "
                }
                s += &String::from(format!("{}\n", self.visit_exec(&e)));
            }
            self.indent -= 1;
            for _ in 0..self.indent {
                s += "    "
            }
            s += "\n)";
            s
        }
        fn visit_exec(&mut self, e: &Exec) -> String {
            match &*e {
                Exec::Left(o) => String::from(format!("!{} ", &self.visit_op(&o))),
                Exec::Right(o) => String::from(format!("{}! ", &self.visit_op(&o))),
            }
        }
        fn visit_op(&mut self, o: &Op) -> String {
            match &*o {
                Op::Instruction(i) => self.visit_instruction(&i),
                Op::Literal(l) => self.visit_literal(&l),
            }
        }
        fn visit_instruction(&mut self, i: &Instruction) -> String {
            String::from(&i.value)
        }
        fn visit_literal(&mut self, l: &Literal) -> String {
            match &*l {
                Literal::Integer(i) => i.to_string(),
                Literal::Float(f) => f.to_string(),
                Literal::Boolean(b) => b.to_string(),
                Literal::Character(c) => format!("\'{}\'", c),
                Literal::List(l) => {
                    let mut s = String::from("[");
                    for e in l {
                        s += &String::from(format!("{}, ", &self.visit_literal(&*e)));
                    }
                    s.push(']');
                    s
                }
                Literal::Block(b) => {
                    let mut s = String::from("{\n");
                    self.indent += 1;
                    for e in b {
                        for _ in 0..self.indent {
                            s += "    "
                        }
                        s += &String::from(format!("{}\n", &self.visit_exec(&e)));
                    }
                    self.indent -= 1;
                    for _ in 0..self.indent {
                        s += "    "
                    }
                    s += "}";
                    s
                }
            }
        }
    }
}

pub mod par {
    use std::usize;

    use crate::lexer::tok;
    use crate::parser::ast;

    type Token = tok::Token;

    enum ParseState {
        Begin,
        InOp,
        InOpFromBang,
        InOpLeft,
        InOpRight,
        InBlockLeft,
        InBlockRight,
        InListLeft,
        InListRight,
        End,
        Error(String),
    }
    pub fn parse_tokens(tokens: &Vec<Token>) -> ast::Code {
        ast::Code::new()
    }
    // pub fn parse(tokens: Vec<Token>) -> ast::Code {
    //     type TokenType = tok::TokenType;
    //     let mut code = ast::Code::new();
    //     let mut parser_state = ParseState::Begin;
    //     let default_token = tok::Token {
    //         token_type: TokenType::End,
    //         string: "".to_string(),
    //         index: usize::MAX,
    //     };

    //     let i = 0;
    //     while i < (&tokens).len() {
    //         let current_token = match (&tokens).get(i) {
    //             Some(c) => c,
    //             None => &default_token,
    //         };
    //         match parser_state {
    //             ParseState::Begin => match current_token.token_type {
    //                 TokenType::BlockBegin => {
    //                     parser_state = ParseState::InBlockLeft;
    //                 }
    //                 TokenType::BlockEnd => {
    //                     parser_state = ParseState::Error(
    //                         format!("ParseBegin @ {}: BlockEnd before any BlockBegin", i)
    //                             .to_string(),
    //                     );
    //                 }
    //                 TokenType::ListBegin => {
    //                     parser_state = ParseState::InListLeft;
    //                 }
    //                 TokenType::ListSep => {
    //                     parser_state = ParseState::Error(
    //                         format!("ParseBegin @ {}: ListSep outside of list", i).to_string(),
    //                     );
    //                 }
    //                 TokenType::ListEnd => {
    //                     parser_state = ParseState::Error(
    //                         format!("ParseBegin @ {}: ListEnd before any ListBegin", i).to_string(),
    //                     );
    //                 }
    //                 TokenType::Instruction => {
    //                     parser_state = ParseState::InOp;
    //                 }
    //                 TokenType::Literal(_) => {
    //                     parser_state = ParseState::InOp;
    //                 }
    //                 TokenType::Bang => {
    //                     parser_state = ParseState::InOpFromBang;
    //                 }
    //                 TokenType::End => {
    //                     if current_token.index == usize::MAX {
    //                         parser_state = ParseState::Error(
    //                             format!("ParseBegin @ {}: out of tokens", i).to_string(),
    //                         );
    //                     } else {
    //                         parser_state = ParseState::End;
    //                     }
    //                 }
    //             },
    //             ParseState::InOp => match current_token.token_type {
    //                 TokenType::Literal(_) => {
    //                     code.push(Box::new(ast::Exec::new(&tokens)));
    //                 }
    //                 TokenType::Instruction => {}
    //                 _ => {}
    //             },
    //             ParseState::InOpFromBang => {}
    //             ParseState::InOpLeft => {}
    //             ParseState::InOpRight => {}
    //             ParseState::InBlockLeft => {}
    //             ParseState::InBlockRight => {}
    //             ParseState::InListLeft => {}
    //             ParseState::InListRight => {}
    //             ParseState::End => {
    //                 break;
    //             }
    //             ParseState::Error(s) => {
    //                 println!("{}", s);
    //                 break;
    //             }
    //         }
    //     }
    //     code
    // }
}
