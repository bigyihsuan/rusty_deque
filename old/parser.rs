pub mod ast1 {
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
    pub type Block = Vec<Box<Exec>>;
}

pub mod par1 {
    use std::usize;

    use crate::lexer::tok::*;
    use crate::parser::ast::*;

    pub fn parse_tokens(tokens: Vec<Token>) -> Code {
        let mut rp = RecursiveParser {
            token_list: tokens,
            current_index: 0,
        };
        rp.code()
    }

    struct RecursiveParser {
        token_list: Vec<Token>,
        current_index: usize,
    }

    // https://craftinginterpreters.com/parsing-expressions.html
    impl RecursiveParser {
        // recursive parsing functions
        /* terminals:
         * Instruction
         * Int
         * Float
         * Bool
         * Char
         */
        pub fn code(&mut self) -> Code {
            // non-terminal
            let mut code: Code = Vec::new();
            while !self.match_tts(vec![TokenType::End]) {
                // println!(
                //     "code {:?} {} @ {}",
                //     self.peek().token_type,
                //     self.peek().string,
                //     self.peek().tok_index
                // );
                if let TokenType::End = self.peek().token_type {
                    break;
                }
                code.push(self.exec());
            }
            code
        }

        pub fn exec(&mut self) -> Exec {
            // non-terminal
            // println!(
            //     "exec before {:?} {} @ {}",
            //     self.peek().token_type,
            //     self.peek().string,
            //     self.peek().tok_index
            // );
            let op = self.op();
            self.advance(); // go past op

            // println!(
            //     "exec before2 {:?} {} @ {}",
            //     self.peek().token_type,
            //     self.peek().string,
            //     self.peek().tok_index
            // );
            match self.peek().token_type {
                TokenType::Bang => {
                    // bang == left exec, advance then return
                    // println!(
                    //     "exec left {:?} {} @ {}",
                    //     self.peek().token_type,
                    //     self.peek().string,
                    //     self.peek().tok_index
                    // );
                    let exec = Exec::Left(op);
                    self.advance();
                    exec
                }
                TokenType::Tilde => {
                    // println!(
                    //     "exec right {:?} {} @ {}",
                    //     self.peek().token_type,
                    //     self.peek().string,
                    //     self.peek().tok_index
                    // );
                    let exec = Exec::Right(op);
                    self.advance();
                    exec
                }
                TokenType::End => {
                    match self.previous().token_type {
                        TokenType::Bang => {
                            // bang == left exec, advance then return
                            // println!(
                            //     "exec left {:?} {} @ {}",
                            //     self.previous().token_type,
                            //     self.previous().string,
                            //     self.previous().tok_index
                            // );
                            let exec = Exec::Left(op);
                            exec
                        }
                        TokenType::Tilde => {
                            // println!(
                            //     "exec right {:?} {} @ {}",
                            //     self.previous().token_type,
                            //     self.previous().string,
                            //     self.previous().tok_index
                            // );
                            let exec = Exec::Right(op);
                            exec
                        }
                        _ => {
                            panic!(
                                "Exec: expected bang or tilde, found {:?} @ {}",
                                self.peek().token_type,
                                self.peek().tok_index
                            )
                        }
                    }
                }
                _ => {
                    panic!(
                        "Exec: expected bang or tilde, found {:?} @ {}",
                        self.peek().token_type,
                        self.peek().tok_index
                    )
                }
            }
        }

        pub fn op(&mut self) -> Op {
            // println!(
            //     "op before {:?} {} @ {}",
            //     self.peek().token_type,
            //     self.peek().string,
            //     self.peek().tok_index
            // );
            // non-terminal
            match self.peek().token_type {
                TokenType::Instruction => {
                    // println!(
                    //     "op inst {:?} {} @ {}",
                    //     self.peek().token_type,
                    //     self.peek().string,
                    //     self.peek().tok_index
                    // );
                    Op::Instruction(self.instruction())
                }
                _ => {
                    // println!(
                    //     "op lit {:?} {} @ {}",
                    //     self.peek().token_type,
                    //     self.peek().string,
                    //     self.peek().tok_index
                    // );
                    Op::Literal(self.literal())
                }
            }
        }

        pub fn instruction(&mut self) -> Instruction {
            // terminal
            // println!(
            //     "inst {:?} {} @ {}",
            //     self.peek().token_type,
            //     self.peek().string,
            //     self.peek().tok_index
            // );
            let i = Instruction {
                value: self.peek().string.to_string(),
            };
            i
        }

        pub fn literal(&mut self) -> Literal {
            // List, Block: non-terminal
            // all else: terminal
            // println!(
            //     "lit {:?} {} @ {}",
            //     self.peek().token_type,
            //     self.peek().string,
            //     self.peek().tok_index
            // );
            let tok = self.peek();
            match &tok.token_type {
                TokenType::Literal(lt) => match lt {
                    LiteralType::Int => Literal::Integer(tok.string.parse().unwrap()),
                    LiteralType::Float => Literal::Float(tok.string.parse().unwrap()),
                    LiteralType::Bool => Literal::Boolean(tok.string.parse().unwrap()),
                    LiteralType::Char => Literal::Character(tok.string.parse().unwrap()),
                    LiteralType::String => {
                        let chars: Vec<Box<Literal>> = tok
                            .string
                            .chars()
                            .map(|c| Box::new(Literal::Character(c)))
                            .collect();
                        Literal::List(chars)
                    }
                },
                TokenType::ListBegin => Literal::List(self.list()),
                TokenType::BlockBegin => Literal::Block(self.block()),
                _ => {
                    panic!(
                        "Literal: expected literal, found {:?} @ {}",
                        tok.token_type, tok.tok_index
                    )
                }
            }
        }

        pub fn list(&mut self) -> List {
            // non-terminal
            // [ element , ... ]
            // println!(
            //     "list {:?} {} @ {}",
            //     self.peek().token_type,
            //     self.peek().string,
            //     self.peek().tok_index
            // );
            let mut list_stack: Vec<usize> = Vec::new();
            list_stack.push(self.peek().tok_index);
            // println!("  stack: {:?}", list_stack);

            let mut list: Vec<Box<Literal>> = Vec::new();
            &mut self.advance(); // skip list start

            while list_stack.len() > 0 {
                // literal, then list sep
                match self.peek().token_type {
                    TokenType::ListBegin => {
                        list_stack.push(self.peek().tok_index);
                    }
                    TokenType::ListEnd => {
                        list_stack.pop();
                    }
                    TokenType::ListSep => {}
                    TokenType::Literal(_) => {
                        list.push(Box::new(self.literal()));
                    }
                    _ => {
                        break;
                    }
                }
                &mut self.advance();
                // println!("  stack: {:?}\n  list:  {:?}", list_stack, list);
            }
            // &mut self.advance(); // skip list end
            // println!("  stack: {:?}\n  list:  {:?}", list_stack, list);
            list
        }

        pub fn block(&mut self) -> Block {
            // { code }
            println!(
                "block {:?} {} @ {}",
                self.peek().token_type,
                self.peek().string,
                self.peek().tok_index
            );
            unimplemented!()
        }

        // utility functions
        fn advance(&mut self) -> &Token {
            // goes to the next token, then returns the previous one
            if !self.is_at_end() {
                self.current_index += 1;
            }
            self.previous()
        }

        // advances if it matches any
        fn match_tts(&mut self, token_types: Vec<TokenType>) -> bool {
            for tt in token_types {
                if self.check_tt(tt) {
                    &self.advance();
                    return true;
                }
            }
            return false;
        }

        fn check_tt(&self, token_type: TokenType) -> bool {
            if self.is_at_end() {
                return false;
            }
            return self.peek().token_type == token_type;
        }

        fn is_at_end(&self) -> bool {
            match self.peek().token_type {
                TokenType::End => true,
                _ => false,
            }
        }

        fn peek(&self) -> &Token {
            match self.token_list.get(self.current_index) {
                Some(t) => t,
                None => panic!(),
            }
        }

        fn previous(&self) -> &Token {
            match self.token_list.get(self.current_index - 1) {
                Some(t) => t,
                None => panic!(),
            }
        }
    }
}
