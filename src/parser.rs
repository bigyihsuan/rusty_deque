pub mod par_ast {
    use std::fmt::Display;

    pub type Code = Vec<Exec>;

    #[derive(Debug, PartialEq, Clone)]
    pub enum Exec {
        Left(Op),
        Right(Op),
    }

    impl Exec {
        pub fn new_left(op: Op) -> Exec {
            Exec::Left(op)
        }
        pub fn new_right(op: Op) -> Exec {
            Exec::Right(op)
        }
    }

    impl Display for Exec {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Exec::Left(op) => write!(f, "{}", op.to_string()),
                Exec::Right(op) => write!(f, "{}", op.to_string()),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Op {
        Literal(Literal),
        Instruction(String),
    }

    impl Op {
        pub fn new_literal(literal: Literal) -> Op {
            Op::Literal(literal)
        }
        pub fn new_instruction(instruction: String) -> Op {
            Op::Instruction(instruction)
        }

        pub fn to_string(&self) -> String {
            match self {
                Op::Literal(literal) => literal.clone().to_string(),
                Op::Instruction(instruction) => instruction.clone(),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Literal {
        Int(i64),
        Float(f64),
        Bool(bool),
        Char(char),
        List(Vec<Literal>),
        Block(Vec<Exec>),
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

        pub fn to_string(self) -> String {
            match self {
                Literal::Int(i) => i.to_string(),
                Literal::Float(f) => f.to_string(),
                Literal::Bool(b) => b.to_string(),
                Literal::Char(c) => format!("'{}'", c),
                Literal::List(ref l) => {
                    let mut is_char_list = true;
                    for elem in l.iter() {
                        if let Literal::Char(_) = elem {
                            continue;
                        } else {
                            is_char_list = false;
                            break;
                        }
                    }
                    if is_char_list {
                        let mut chars = '"'.to_string();
                        for elem in l.iter() {
                            if let Literal::Char(c) = elem {
                                chars.push(*c);
                            }
                        }
                        chars.push('"');
                        chars
                    } else {
                        // extract each element in the list
                        // and put it in square brackets, comma separated
                        let mut s = "[".to_string();
                        for lit in l {
                            s.push_str(&lit.clone().to_string());
                            s.push_str(", ");
                        }
                        s.push_str("]");
                        s
                    }
                }
                Literal::Block(b) => {
                    // do the same thing as list, but execs print their lexeme and sigil
                    let mut s = "{".to_string();
                    for exec in b {
                        match exec {
                            Exec::Left(op) => {
                                s.push_str(op.to_string().as_str());
                                s.push_str("!");
                            }
                            Exec::Right(op) => {
                                s.push_str(op.to_string().as_str());
                                s.push_str("~");
                            }
                        }
                        s.push_str(" ");
                    }
                    s.push_str("}");
                    s
                }
            }
        }
    }

    // impl Display for Literal {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         match self {
    //             Literal::Int(value) => write!(f, "{}", value),
    //             Literal::Float(value) => write!(f, "{}", value),
    //             Literal::Bool(value) => write!(f, "{}", value),
    //             Literal::Char(value) => write!(f, "{}", value),
    //             Literal::List(value) => write!(f, "{:?}", value),
    //             Literal::Block(value) => write!(f, "{:?}", value),
    //         }
    //     }
    // }
}

pub mod par {

    use crate::lexer::lex_token::{Token, TokenType};
    use crate::parser::par_ast::*;
    use std::vec;

    // type CodeResult = Result<Code, String>;
    type ExecResult = Result<Exec, String>;
    type OpResult = Result<Op, String>;
    type LiteralResult = Result<Literal, String>;

    // parses an input vec of tokens into an ast, with root at Code
    pub fn parse_tokens(tokens: &mut vec::IntoIter<Token>) -> Result<Code, String> {
        let mut code: Code = Vec::new();
        // while ther are tokens left, parse execs by calling parse_exec
        while tokens.into_iter().len() > 0 {
            let exec = parse_exec(tokens);
            match exec {
                Ok(e) => code.push(e),
                Err(e) => return Err(e),
            }
        }
        Ok(code)
    }

    // parses a list of tokens into an Exec
    pub fn parse_exec(tokens: &mut vec::IntoIter<Token>) -> ExecResult {
        // println!("parse_exec {:?}", &tokens);
        let op = parse_op(tokens);
        match op {
            Ok(op) => {
                let mut iter = tokens.peekable();
                let sigil = iter.peek().unwrap();
                // println!("    exec sigil {:?}", &sigil);
                match sigil.token_type {
                    TokenType::Bang => Ok(Exec::new_left(op)),
                    TokenType::Tilde => Ok(Exec::new_right(op)),
                    _ => Err(format!(
                        "Parser Error: Expected sigil Bang or Tilde, instead got {:?}",
                        sigil.token_type
                    )),
                }
            }
            Err(e) => Err(e),
        }
    }

    // parses a list of tokens into an Op
    pub fn parse_op(tokens: &mut vec::IntoIter<Token>) -> OpResult {
        // println!("parse_op {:?}", &tokens);
        let op_token = tokens.next().unwrap();
        // println!("    op_token {:?}", &op_token);
        match op_token.token_type {
            TokenType::ConstInt
            | TokenType::ConstFloat
            | TokenType::ConstChar
            | TokenType::ConstString
            | TokenType::ConstBool => {
                let parsed = parse_literal(&op_token);
                match parsed {
                    Ok(lit) => Ok(Op::new_literal(lit)),
                    Err(e) => Err(format!("Parser Error: {} for {:?}", e, op_token)),
                }
            }
            TokenType::LeftSquare => {
                let parsed = parse_list(tokens, true);
                match parsed {
                    Ok(list) => Ok(Op::new_literal(list)),
                    Err(e) => Err(format!("Parser Error: {} for {:?}", e, op_token)),
                }
            }
            TokenType::LeftCurly => {
                let parsed = parse_block(tokens);
                match parsed {
                    Ok(block) => Ok(Op::new_literal(block)),
                    Err(e) => Err(format!("Parser Error: {} for {:?}", e, op_token)),
                }
            }
            TokenType::Instr => Ok(Op::new_instruction(op_token.lexeme.clone())),
            tt => Err(format!(
                "Parse Error: Unexpected token type {:?} for Op",
                tt
            )),
        }
    }

    // parses a list of tokens into a list literal
    // returns the list literal and the remaining tokens
    // https://stackoverflow.com/questions/60087757/passing-an-iterator-into-a-recursive-call-during-an-iteration-in-rust
    pub fn parse_list(tokens: &mut vec::IntoIter<Token>, nested: bool) -> LiteralResult {
        let iter = tokens;
        let mut list: Vec<Literal> = vec![];
        let mut ended_last_list = false;

        // skip initial left square
        if !nested {
            iter.next();
        }
        while let Some(token) = iter.next() {
            // panic if the input ends before a closing square bracket
            // println!("current token: {:?}", token);
            match token.token_type {
                // if see another list, recurse into another list
                TokenType::LeftSquare => {
                    // println!("    making nested list at {:?}", token);
                    ended_last_list = false;
                    // make a new list
                    let parsed = parse_list(iter, true);
                    match parsed {
                        Ok(list_lit) => {
                            // println!("    nested list {:?}", list_lit);
                            list.push(list_lit);
                        }
                        Err(e) => return Err(format!("Parser Error: {} for {:?}", e, token)),
                    }
                }
                // ignore commas
                TokenType::Comma => {
                    continue;
                }
                // finish the current list
                TokenType::RightSquare => {
                    // println!("    closing this list");
                    ended_last_list = true;
                    break;
                }
                // otherwise, parse the literal and add it to the list
                _ => {
                    let parsed = parse_literal(&token);
                    match parsed {
                        Ok(lit) => list.push(lit),
                        Err(e) => return Err(format!("Parser Error: {} for {:?}", e, token)),
                    }
                }
            }
        }
        // if the list did not close, panic
        if !ended_last_list {
            return Err(format!("Parsing Error: Unclosed list"));
        }
        Ok(Literal::new_list(list))
    }

    pub fn parse_block(tokens: &mut vec::IntoIter<Token>) -> LiteralResult {
        // println!("parse_block {:?}", &tokens);
        let mut block: Vec<Exec> = vec![];
        while tokens.len() > 0 {
            // println!("bef----  {:?}", &tokens);
            let mut iter = tokens.clone();
            if let Some(Token {
                token_type: TokenType::RightCurly,
                ..
            }) = iter.next()
            {
                break;
            }
            let exec = parse_exec(tokens);
            match exec {
                Ok(e) => block.push(e),
                Err(e) => return Err(format!("Parser Error: {} for {:?}", e, tokens)),
            }
            // println!("aft----  {:?}", &tokens);
        }
        // println!("finish--  {:?}", &tokens);
        tokens.next();
        Ok(Literal::new_block(block))
    }

    // parses a literal token into a literal
    pub fn parse_literal(token: &Token) -> LiteralResult {
        match token.token_type {
            TokenType::ConstInt => Ok(Literal::new_int(token.lexeme.parse::<i64>().unwrap())),
            TokenType::ConstFloat => Ok(Literal::new_float(token.lexeme.parse::<f64>().unwrap())),
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
                        'n' => Ok(Literal::new_char('\n')),
                        'r' => Ok(Literal::new_char('\r')),
                        't' => Ok(Literal::new_char('\t')),
                        '\\' => Ok(Literal::new_char('\\')),
                        '\'' => Ok(Literal::new_char('\'')),
                        '\"' => Ok(Literal::new_char('\"')),
                        '0' => Ok(Literal::new_char('\0')),
                        _ => Err(format!(
                            "Parser Error: Unrecognized character escape sequence `{}`",
                            chars
                        )),
                    }
                } else {
                    Ok(Literal::new_char(chars.parse::<char>().unwrap()))
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
                            _ => {
                                return Err(format!(
                                    "Parser Error: Unrecognized character escape sequence `{}`",
                                    chars
                                ))
                            }
                        });
                    } else {
                        string_chars.push(Literal::new_char(c));
                    }
                }
                Ok(Literal::new_list(string_chars))
            }
            TokenType::ConstBool => {
                if token.lexeme == "true" {
                    Ok(Literal::new_bool(true))
                } else {
                    Ok(Literal::new_bool(false))
                }
            }
            _ => {
                return Err(format!(
                    "Parser Error: Unexpected token type {:?} for Literal",
                    token.token_type
                ))
            }
        }
    }
}
