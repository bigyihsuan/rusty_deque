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

    #[derive(Debug)]
    pub enum BlockElement {
        Block(Block),
        Exec(Box<Exec>),
    }
    pub type Block = Vec<BlockElement>;
}

pub mod par {
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
                println!(
                    "code {:?} {} @ {}",
                    self.peek().token_type,
                    self.peek().string,
                    self.peek().tok_index
                );
                if let TokenType::End = self.peek().token_type {
                    break;
                }
                code.push(self.exec());
            }
            code
        }

        pub fn exec(&mut self) -> Exec {
            // non-terminal
            println!(
                "exec before {:?} {} @ {}",
                self.peek().token_type,
                self.peek().string,
                self.peek().tok_index
            );
            match self.peek().token_type {
                TokenType::Bang => {
                    // bang == left exec, advance then return
                    println!(
                        "exec left {:?} {} @ {}",
                        self.peek().token_type,
                        self.peek().string,
                        self.peek().tok_index
                    );
                    self.advance();
                    Exec::Left(self.op())
                }
                _ => {
                    println!(
                        "exec right {:?} {} @ {}",
                        self.peek().token_type,
                        self.peek().string,
                        self.peek().tok_index
                    );
                    // no bang == right exec, advance to get the op, then advance past bang
                    let o = Exec::Right(self.op());
                    self.advance();
                    o
                }
            }
        }

        pub fn op(&mut self) -> Op {
            println!(
                "op before {:?} {} @ {}",
                self.peek().token_type,
                self.peek().string,
                self.peek().tok_index
            );
            // non-terminal
            match self.peek().token_type {
                TokenType::Instruction => {
                    println!(
                        "op inst {:?} {} @ {}",
                        self.peek().token_type,
                        self.peek().string,
                        self.peek().tok_index
                    );
                    Op::Instruction(self.instruction())
                }
                _ => {
                    println!(
                        "op lit {:?} {} @ {}",
                        self.peek().token_type,
                        self.peek().string,
                        self.peek().tok_index
                    );
                    Op::Literal(self.literal())
                }
            }
        }

        pub fn instruction(&mut self) -> Instruction {
            // terminal
            println!(
                "inst {:?} {} @ {}",
                self.peek().token_type,
                self.peek().string,
                self.peek().tok_index
            );
            let i = Instruction {
                value: self.peek().string.to_string(),
            };
            self.advance();
            i
        }

        pub fn literal(&mut self) -> Literal {
            // List, Block: non-terminal
            // all else: terminal
            println!(
                "lit {:?} {} @ {}",
                self.peek().token_type,
                self.peek().string,
                self.peek().tok_index
            );
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
            println!(
                "list {:?} {} @ {}",
                self.peek().token_type,
                self.peek().string,
                self.peek().tok_index
            );
            let mut list_stack: Vec<usize> = Vec::new();
            list_stack.push(self.peek().tok_index);
            println!("  stack: {:?}", list_stack);

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
                println!("  stack: {:?}\n  list:  {:?}", list_stack, list);
            }
            // &mut self.advance(); // skip list end
            println!("  stack: {:?}\n  list:  {:?}", list_stack, list);
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

            let mut block_code = Block::new();
            let mut block_stack: Vec<Block> = Vec::new();

            loop {
                println!(
                    "block making {:?} {} @ {}",
                    self.peek().token_type,
                    self.peek().string,
                    self.peek().tok_index
                );
                println!("stack before {:?}", block_stack);
                match self.peek().token_type {
                    TokenType::BlockBegin => {
                        block_stack.push(Block::new());
                        self.advance();
                    }
                    TokenType::BlockEnd => {
                        let current_block = block_stack.pop().unwrap();
                        if block_stack.len() > 0 {
                            block_code.push(BlockElement::Block(current_block));
                        }
                    }
                    _ => {
                        let mut current_block = block_stack.pop().unwrap();
                        current_block.push(BlockElement::Exec(Box::new(self.exec())));
                        block_stack.push(current_block);
                    }
                }
                println!("stack after {:?}", block_stack);
                println!("{:?}", block_code);

                if block_stack.len() == 0 {
                    break;
                }
            }

            block_code
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
