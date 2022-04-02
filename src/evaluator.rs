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
    use std::collections::VecDeque;

    // DEQUE OPS
    pub fn pop(deque: &mut VecDeque<Value>, place: Place) {
        match place {
            Place::Left => {
                deque.pop_front();
            }
            Place::Right => {
                deque.pop_back();
            }
        }
    }

    pub fn dup(stack: &mut VecDeque<Value>, place: Place) {
        let literal = match place {
            Place::Left => stack.pop_front().unwrap(),
            Place::Right => stack.pop_back().unwrap(),
        };
        stack.push_front(literal.clone());
        stack.push_front(literal);
    }

    pub fn rot(deque: &mut VecDeque<Value>, place: Place) {
        match place {
            Place::Left => {
                deque.rotate_left(1);
            }
            Place::Right => {
                deque.rotate_right(1);
            }
        }
    }

    pub fn over(deque: &mut VecDeque<Value>, place: Place) {
        let mut iter = deque.iter();
        match place {
            Place::Left => {
                iter.next();
                let ele = iter.next().unwrap().clone();
                deque.push_front(ele);
            }
            Place::Right => {
                iter.next_back();
                let ele = iter.next_back().unwrap().clone();
                deque.push_back(ele);
            }
        }
    }

    // IO

    pub fn ol(stack: &mut VecDeque<Value>, place: Place) {
        let literal = match place {
            Place::Left => stack.pop_front().unwrap(),
            Place::Right => stack.pop_back().unwrap(),
        };
        // if all elements in literal are Literal::Char, then print them in double quotes instead of as a list
        let mut is_char_list = true;
        match literal {
            Literal::List(ref list) => {
                for elem in list.iter() {
                    if let Literal::Char(_) = elem {
                        continue;
                    } else {
                        is_char_list = false;
                        break;
                    }
                }
                if is_char_list {
                    let mut chars = String::new();
                    for elem in list.iter() {
                        if let Literal::Char(c) = elem {
                            chars.push(*c);
                        }
                    }
                    println!("{}", chars);
                } else {
                    println!("{}", literal.to_string());
                }
            }
            _ => println!("{}", literal.to_string()),
        };
    }

    // pretty much the same as ol, consider consolidating
    pub fn ow(stack: &mut VecDeque<Value>, place: Place) {
        let literal = match place {
            Place::Left => stack.pop_front().unwrap(),
            Place::Right => stack.pop_back().unwrap(),
        };
        // if all elements in literal are Literal::Char, then print them in double quotes instead of as a list
        let mut is_char_list = true;
        match literal {
            Literal::List(ref list) => {
                for elem in list.iter() {
                    if let Literal::Char(_) = elem {
                        continue;
                    } else {
                        is_char_list = false;
                        break;
                    }
                }
                if is_char_list {
                    let mut chars = String::new();
                    for elem in list.iter() {
                        if let Literal::Char(c) = elem {
                            chars.push(*c);
                        }
                    }
                    print!("{}", chars);
                } else {
                    print!("{}", literal.to_string());
                }
            }
            _ => print!("{}", literal.to_string()),
        };
    }
}

pub mod eval {
    use super::eval_value::*;
    use crate::evaluator::eval_instr::*;
    use crate::parser::par_ast::*;

    use std::collections::VecDeque;

    pub fn run_ast(deque: Option<VecDeque<Value>>, ast: Code) -> VecDeque<Value> {
        let temp: VecDeque<Value> = VecDeque::new();
        let d: &mut VecDeque<Value> = &mut deque.unwrap_or(temp);

        for exec in ast {
            match exec {
                Exec::Left(op) => match op {
                    Op::Literal(lit) => {
                        d.push_front(lit);
                    }
                    Op::Instruction(instruction) => call_instr(d, instruction, Place::Left),
                },
                Exec::Right(op) => match op {
                    Op::Literal(lit) => {
                        d.push_back(lit);
                    }
                    Op::Instruction(instruction) => call_instr(d, instruction, Place::Right),
                },
            }
        }
        d.to_owned()
    }

    pub fn call_instr(deque: &mut VecDeque<Value>, instr: String, place: Place) {
        match instr.as_str() {
            // DEQUE OPS
            "pop" => pop(deque, place),
            "dup" => dup(deque, place),
            "rot" => rot(deque, place),
            "over" => over(deque, place),
            // IO
            "ol" => ol(deque, place),
            "ow" => ow(deque, place),
            _ => panic!("Unknown instruction: {}", instr),
        }
    }
}