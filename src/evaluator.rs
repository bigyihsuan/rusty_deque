pub mod eval_value {
    use crate::parser::par_ast::*;

    pub type Value = Literal;

    pub enum Place {
        Left,
        Right,
    }
}

pub mod eval_instr {
    use super::eval_value::*;
    use crate::parser::par_ast::*;
    use std::{collections::VecDeque, fmt::format};

    pub fn dup(stack: VecDeque<Value>, place: Place) {
        let literal = match place {
            Place::Left => stack.pop_front().unwrap(),
            Place::Right => stack.pop_back().unwrap(),
        };
        stack.push_front(literal.clone());
        stack.push_front(literal);
    }

    pub fn ol(stack: VecDeque<Value>, place: Place) {
        let literal = match place {
            Place::Left => stack.pop_front().unwrap(),
            Place::Right => stack.pop_back().unwrap(),
        };
        let value = format!("{}", literal);
        println!("{}", value);
    }
}

pub mod eval {
    use super::eval_value::*;
    use crate::evaluator::eval_instr::*;
    use crate::parser::par_ast::*;

    use std::collections::{HashMap, VecDeque};

    // https://stackoverflow.com/a/27582993/8143168
    macro_rules! collection {
        // map-like
        ($($k:expr => $v:expr),* $(,)?) => {{
            core::convert::From::from([$(($k, $v),)*])
        }};
        // set-like
        ($($v:expr),* $(,)?) => {{
            core::convert::From::from([$($v,)*])
        }};
    }

    const instruction_code: HashMap<String, fn(&mut VecDeque<Value>, Place)> = collection![
        "dup" => crate::evaluator::eval_instr::dup
    ];

    pub fn run_ast(ast: Code) {
        let mut deque: VecDeque<Value> = VecDeque::new();

        for exec in ast {
            match exec {
                Exec::Left(op) => match op {
                    Op::Literal(lit) => {
                        deque.push_front(lit);
                    }
                    Op::Instruction(instruction) => {
                        unimplemented!("{:?}", instruction);
                    }
                },
                Exec::Right(op) => match op {
                    Op::Literal(lit) => {
                        deque.push_back(lit);
                    }
                    Op::Instruction(instruction) => {
                        unimplemented!("{:?}", instruction);
                    }
                },
            }
        }

        for value in deque {
            println!("{:?}", value);
        }
    }
}
