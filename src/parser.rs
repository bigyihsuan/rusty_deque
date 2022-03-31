pub mod par_ast {
    pub type Code = Vec<Exec>;

    #[derive(Debug, PartialEq)]
    pub enum Exec {
        Left(Op),
        Right(Op),
    }

    #[derive(Debug, PartialEq)]
    pub enum Op {
        Literal(Literal),
        Instruction(String),
    }

    #[derive(Debug, PartialEq)]
    pub enum Literal {
        Int(i64),
        Float(f64),
        Bool(bool),
        Char(char),
        List(Vec<Literal>),
        Block(Vec<Exec>),
    }

    impl Exec {
        pub fn is_left(&self) -> bool {
            match self {
                Exec::Left(_) => true,
                _ => false,
            }
        }

        pub fn is_right(&self) -> bool {
            match self {
                Exec::Right(_) => true,
                _ => false,
            }
        }

        pub fn new_left(op: Op) -> Exec {
            Exec::Left(op)
        }
        pub fn new_right(op: Op) -> Exec {
            Exec::Right(op)
        }
    }

    impl Op {
        pub fn is_literal(&self) -> bool {
            match self {
                Op::Literal(_) => true,
                _ => false,
            }
        }

        pub fn is_instruction(&self) -> bool {
            match self {
                Op::Instruction(_) => true,
                _ => false,
            }
        }

        pub fn new_literal(literal: Literal) -> Op {
            Op::Literal(literal)
        }
        pub fn new_instruction(instruction: String) -> Op {
            Op::Instruction(instruction)
        }
    }

    impl Literal {
        pub fn new_int(value: i64) -> Literal {
            Literal::Int(value)
        }
        pub fn new_float(value: f64) -> Literal {
            Literal::Float(value)
        }
        pub fn new_bool(value: bool) -> Literal {
            Literal::Bool(value)
        }
        pub fn new_char(value: char) -> Literal {
            Literal::Char(value)
        }
        pub fn new_list(value: Vec<Literal>) -> Literal {
            Literal::List(value)
        }
        pub fn new_block(value: Vec<Exec>) -> Literal {
            Literal::Block(value)
        }
    }
}

pub mod par {

    use crate::lexer::lex_token::{Token, TokenType};
    use crate::parser::par_ast::*;
    use std::vec;

    enum ParserState {
        Start,
        InList,
    }

    // parses an input vec of tokens into an ast, with root at Code
    pub fn parse_tokens(tokens: &mut vec::IntoIter<Token>) -> Code {
        let mut code: Code = Vec::new();
        // while ther are tokens left, parse execs by calling parse_exec
        while tokens.into_iter().len() > 0 {
            let exec = parse_exec(tokens);
            code.push(exec);
        }
        code
    }

    // parses a list of tokens into an Exec
    pub fn parse_exec(tokens: &mut vec::IntoIter<Token>) -> Exec {
        println!("parse_exec {:?}", &tokens);

        let op = parse_op(tokens);
        let mut iter = tokens.peekable();
        let sigil = iter.peek().unwrap();
        match sigil.token_type {
            TokenType::Bang => Exec::new_left(op),
            TokenType::Tilde => Exec::new_right(op),
            _ => panic!(
                "Parser Error: Expected sigil Bang or Tilde, instead got {:?}",
                sigil.token_type
            ),
        }
    }

    // parses a list of tokens into an Op
    pub fn parse_op(tokens: &mut vec::IntoIter<Token>) -> Op {
        println!("parse_op {:?}", &tokens);

        let mut iter = tokens.peekable();
        let op_token = iter.peek().unwrap().to_owned();
        match op_token.token_type {
            TokenType::ConstInt
            | TokenType::ConstFloat
            | TokenType::ConstChar
            | TokenType::ConstString
            | TokenType::ConstBool => Op::new_literal(parse_literal(&op_token)),
            TokenType::LeftSquare => {
                // iter.;
                Op::new_literal(parse_list(tokens, true))
            }
            TokenType::Instr => Op::new_instruction(op_token.lexeme),
            tt => panic!("Parse Error: Unexpected token type {:?} for Op", tt),
        }
    }

    // parses a list of tokens into a list literal
    // returns the list literal and the remaining tokens
    // https://stackoverflow.com/questions/60087757/passing-an-iterator-into-a-recursive-call-during-an-iteration-in-rust
    pub fn parse_list(tokens: &mut vec::IntoIter<Token>, nested: bool) -> Literal {
        let iter = tokens;
        let mut list: Vec<Literal> = vec![];
        let mut ended_last_list = false;

        // skip initial left square
        if !nested {
            iter.next();
        }
        while let Some(token) = iter.next() {
            // panic if the input ends before a closing square bracket
            println!("current token: {:?}", token);
            match token.token_type {
                // if see another list, recurse into another list
                TokenType::LeftSquare => {
                    println!("    making nested list at {:?}", token);
                    ended_last_list = false;
                    // make a new list
                    list.push(parse_list(iter, true));
                }
                // ignore commas
                TokenType::Comma => {
                    continue;
                }
                // finish the current list
                TokenType::RightSquare => {
                    println!("    closing this list");
                    ended_last_list = true;
                    break;
                }
                // otherwise, parse the literal and add it to the list
                _ => {
                    list.push(parse_literal(&token));
                }
            }
        }
        // if the list did not close, panic
        if !ended_last_list {
            panic!("Parsing Error: Unclosed list");
        }
        Literal::new_list(list)
    }

    pub fn parse_block(tokens: &mut vec::IntoIter<Token>, nested: bool) -> Literal {
        let iter = tokens;
        let mut block: Vec<Literal> = vec![];
        let mut ended_last_block = false;

        // skip initial left square
        if !nested {
            iter.next();
        }
        while let Some(token) = iter.next() {
            // panic if the input ends before a closing square bracket
            println!("current token: {:?}", token);
            match token.token_type {
                // if see another list, recurse into another list
                TokenType::LeftSquare => {
                    println!("    making nested block at {:?}", token);
                    ended_last_block = false;
                    // make a new list
                    block.push(parse_block(iter, true));
                }
                // ignore commas
                TokenType::Comma => {
                    continue;
                }
                // finish the current list
                TokenType::RightSquare => {
                    println!("    closing this block");
                    ended_last_block = true;
                    break;
                }
                // otherwise, parse the literal and add it to the list
                _ => {
                    block.push(parse_exec(&iter));
                }
            }
        }
        // if the list did not close, panic
        if !ended_last_block {
            panic!("Parsing Error: Unclosed block");
        }
        Literal::new_list(block)
    }

    // parses a literal token into a literal
    pub fn parse_literal(token: &Token) -> Literal {
        match token.token_type {
            TokenType::ConstInt => Literal::new_int(token.lexeme.parse::<i64>().unwrap()),
            TokenType::ConstFloat => Literal::new_float(token.lexeme.parse::<f64>().unwrap()),
            TokenType::ConstChar => {
                // strip quotes
                let mut chars = token.lexeme.chars();
                chars.next();
                chars.next_back();
                let chars: String = chars.collect();

                // handle escaped characters, which start with a forwards slash
                if chars.starts_with('\\') {
                    let c = chars.chars().nth(1).unwrap();
                    match c {
                        'n' => Literal::new_char('\n'),
                        'r' => Literal::new_char('\r'),
                        't' => Literal::new_char('\t'),
                        '\\' => Literal::new_char('\\'),
                        '\'' => Literal::new_char('\''),
                        '\"' => Literal::new_char('\"'),
                        '0' => Literal::new_char('\0'),
                        _ => panic!(
                            "Parser Error: Unrecognized character escape sequence `{}`",
                            chars
                        ),
                    }
                } else {
                    Literal::new_char(chars.parse::<char>().unwrap())
                }
            }
            TokenType::ConstString => {
                // strip quotes
                let mut chars = token.lexeme.chars();
                chars.next();
                chars.next_back();
                let chars: String = chars.collect();

                let mut string_chars: Vec<Literal> = vec![];
                let mut iter = chars.chars();
                while let Some(c) = iter.next() {
                    // handle escaped characters, which start with a forwards slash
                    if c == '\\' {
                        // take another character
                        let c = iter.next().unwrap();
                        string_chars.push(match c {
                            'n' => Literal::new_char('\n'),
                            'r' => Literal::new_char('\r'),
                            't' => Literal::new_char('\t'),
                            '\\' => Literal::new_char('\\'),
                            '\'' => Literal::new_char('\''),
                            '\"' => Literal::new_char('\"'),
                            '0' => Literal::new_char('\0'),
                            _ => panic!(
                                "Parser Error: Unrecognized character escape sequence `{}`",
                                chars
                            ),
                        });
                    } else {
                        string_chars.push(Literal::new_char(c));
                    }
                }
                Literal::new_list(string_chars)
            }
            TokenType::ConstBool => {
                if token.lexeme == "true" {
                    Literal::new_bool(true)
                } else {
                    Literal::new_bool(false)
                }
            }
            _ => panic!(
                "Parser Error: Unexpected token type {:?} for Literal",
                token.token_type
            ),
        }
    }
}
