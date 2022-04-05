pub mod eval_value {
    use crate::parser::par_ast::*;

    pub type Value = Literal;

    pub enum Place {
        Left,
        Right,
    }
}

pub mod eval_instr {
    use super::{eval::*, eval_value::*};
    use crate::{
        lexer::lex::tokenize_code,
        parser::{par::parse_literal, par_ast::*},
    };
    use std::{
        collections::VecDeque,
        io::{self, Read},
    };

    type ValResult = Result<Value, &'static str>;
    type Nullary = fn() -> ValResult;
    type Unary = fn(a: Value) -> ValResult;
    type Binary = fn(a: Value, b: Value) -> ValResult;
    type Ternary = fn(a: Value, b: Value, b: Value) -> ValResult;

    // bool is to push the result back to the stack
    pub fn unary(deque: &mut VecDeque<Value>, place: Place, func: Unary, push_result: bool) {
        match place {
            Place::Left => {
                let val = deque.pop_front().unwrap();
                let result = func(val);
                if push_result {
                    match result {
                        Ok(v) => deque.push_front(v),
                        Err(e) => println!("{}", e),
                    }
                }
            }
            Place::Right => {
                let val = deque.pop_back().unwrap();
                let result = func(val);
                if push_result {
                    match result {
                        Ok(v) => deque.push_back(v),
                        Err(e) => println!("{}", e),
                    }
                }
            }
        }
    }

    pub fn binary(deque: &mut VecDeque<Value>, place: Place, func: Binary, push_result: bool) {
        match place {
            Place::Left => {
                let val_a = deque.pop_front().unwrap();
                let val_b = deque.pop_front().unwrap();
                let result = func(val_a, val_b);
                if push_result {
                    match result {
                        Ok(v) => deque.push_front(v),
                        Err(e) => println!("{}", e),
                    }
                }
            }
            Place::Right => {
                let val_a = deque.pop_back().unwrap();
                let val_b = deque.pop_back().unwrap();
                let result = func(val_a, val_b);
                if push_result {
                    match result {
                        Ok(v) => deque.push_back(v),
                        Err(e) => println!("{}", e),
                    }
                }
            }
        }
    }
    pub fn ternary(deque: &mut VecDeque<Value>, place: Place, func: Ternary, push_result: bool) {
        match place {
            Place::Left => {
                let val_a = deque.pop_front().unwrap();
                let val_b = deque.pop_front().unwrap();
                let val_c = deque.pop_front().unwrap();
                let result = func(val_a, val_b, val_c);
                if push_result {
                    match result {
                        Ok(v) => deque.push_front(v),
                        Err(e) => println!("{}", e),
                    }
                }
            }
            Place::Right => {
                let val_a = deque.pop_back().unwrap();
                let val_b = deque.pop_back().unwrap();
                let val_c = deque.pop_back().unwrap();
                let result = func(val_a, val_b, val_c);
                if push_result {
                    match result {
                        Ok(v) => deque.push_back(v),
                        Err(e) => println!("{}", e),
                    }
                }
            }
        }
    }

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

    pub fn swap(deque: &mut VecDeque<Value>, place: Place) {
        match place {
            Place::Left => {
                let val_a = deque.pop_front().unwrap();
                let val_b = deque.pop_front().unwrap();
                deque.push_front(val_a);
                deque.push_front(val_b);
            }
            Place::Right => {
                let val_a = deque.pop_back().unwrap();
                let val_b = deque.pop_back().unwrap();
                deque.push_back(val_a);
                deque.push_back(val_b);
            }
        }
    }

    pub fn len(deque: &mut VecDeque<Value>, place: Place) {
        match place {
            Place::Left => {
                let len = deque.len();
                deque.push_front(Value::Int(len as i64));
            }
            Place::Right => {
                let len = deque.len();
                deque.push_back(Value::Int(len as i64));
            }
        }
    }
    // CASTINGS
    pub fn cast_to_int(val: Value) -> ValResult {
        match val {
            Value::Int(i) => Ok(Value::Int(i)),
            Value::Float(f) => Ok(Value::Int(f as i64)),
            Value::Bool(b) => Ok(Value::Int(if b { 1 } else { 0 })),
            Value::Char(c) => Ok(Value::Int(c as i64)),
            _ => Err("Cannot cast to int"),
        }
    }
    pub fn cast_to_float(val: Value) -> ValResult {
        match val {
            Value::Int(i) => Ok(Value::Float(i as f64)),
            Value::Float(f) => Ok(Value::Float(f)),
            Value::Bool(b) => Ok(Value::Float(if b { 1.0 } else { 0.0 })),
            Value::Char(c) => Ok(Value::Float(c as i64 as f64)),
            _ => Err("Cannot cast to float"),
        }
    }
    pub fn cast_to_bool(val: Value) -> ValResult {
        match val {
            Value::Int(_) => Ok(Value::Bool(truthiness_of(val))),
            Value::Float(_) => Ok(Value::Bool(truthiness_of(val))),
            Value::Bool(_) => Ok(Value::Bool(truthiness_of(val))),
            Value::Char(_) => Ok(Value::Bool(truthiness_of(val))),
            Value::List(_) => Ok(Value::Bool(truthiness_of(val))),
            Value::Block(_) => Ok(Value::Bool(truthiness_of(val))),
        }
    }
    pub fn cast_to_char(val: Value) -> ValResult {
        match val {
            Value::Int(i) => Ok(Value::Char(i as u8 as char)),
            Value::Float(f) => Ok(Value::Char(f as u8 as char)),
            Value::Bool(b) => Ok(Value::Char(if b { '1' } else { '0' })),
            Value::Char(c) => Ok(Value::Char(c)),
            _ => Err("Cannot cast to char"),
        }
    }

    // INT/FLOAT OPS
    pub fn add(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            _ => Err("invalid operands for addition"),
        }
    }
    pub fn sub(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            _ => Err("invalid operands for subtraction"),
        }
    }
    pub fn mult(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            _ => Err("invalid operands for multiplication"),
        }
    }
    pub fn intdiv(a: Value, b: Value) -> ValResult {
        if b == Value::Int(0) || b == Value::Float(0.0) {
            return Err("integer division by zero");
        }
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 / b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            _ => Err("invalid operands for integer division"),
        }
    }
    pub fn floatdiv(a: Value, b: Value) -> ValResult {
        if b == Value::Int(0) || b == Value::Float(0.0) {
            return Err("float division by zero");
        }
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Float(a as f64 / b as f64)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 / b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            _ => Err("invalid operands for float division"),
        }
    }
    pub fn modulo(a: Value, b: Value) -> ValResult {
        if b == Value::Int(0) || b == Value::Float(0.0) {
            return Err("modulo by zero");
        }
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a % b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 % b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a % b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
            _ => Err("invalid operands for modulo"),
        }
    }
    pub fn exp(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Float((a as f64).powf(b as f64))),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f64).powf(b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.powf(b as f64))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(b))),
            _ => Err("invalid operands for exponentiation"),
        }
    }
    pub fn log(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Float((a as f64).log(b as f64))),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f64).log(b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.log(b as f64))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.log(b))),
            _ => Err("invalid operands for logarithm"),
        }
    }
    pub fn neg(a: Value) -> ValResult {
        match a {
            Value::Int(a) => Ok(Value::Int(-a)),
            Value::Float(a) => Ok(Value::Float(-a)),
            _ => Err("invalid operand for negation"),
        }
    }
    // bitwise ops
    // all of these act directly on the bits
    // if any are floats get the bits of those floats
    // all of these return an int
    pub fn bitand(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a & b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Int(((a as u64) & b.to_bits()) as i64)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Int((a.to_bits() & (b as u64)) as i64)),
            (Value::Float(a), Value::Float(b)) => {
                Ok(Value::Int((a.to_bits() & b.to_bits()) as i64))
            }
            _ => Err("invalid operands for bitwise AND"),
        }
    }
    pub fn bitor(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a | b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Int(((a as u64) | b.to_bits()) as i64)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Int((a.to_bits() | (b as u64)) as i64)),
            (Value::Float(a), Value::Float(b)) => {
                Ok(Value::Int((a.to_bits() | b.to_bits()) as i64))
            }
            _ => Err("invalid operands for bitwise OR"),
        }
    }
    pub fn bitxor(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a ^ b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Int(((a as u64) ^ b.to_bits()) as i64)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Int((a.to_bits() ^ (b as u64)) as i64)),
            (Value::Float(a), Value::Float(b)) => {
                Ok(Value::Int((a.to_bits() ^ b.to_bits()) as i64))
            }
            _ => Err("invalid operands for bitwise XOR"),
        }
    }
    pub fn bitnot(a: Value) -> ValResult {
        match a {
            Value::Int(a) => Ok(Value::Int(!a)),
            Value::Float(a) => Ok(Value::Int(!(a.to_bits()) as i64)),
            _ => Err("invalid operand for bitwise NOT"),
        }
    }
    // COMPARISON
    // all the inputs can be char, int, or float
    // all of these return a bool
    pub fn eq(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a == b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 == b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a == b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a == b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a == b)),
            _ => Err("invalid operands for equality"),
        }
    }
    pub fn neq(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a != b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 != b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a != b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a != b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a != b)),
            _ => Err("invalid operands for inequality"),
        }
    }
    pub fn lt(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) < b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a < b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a < b)),
            _ => Err("invalid operands for less than"),
        }
    }
    pub fn gt(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 > b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a > b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a > b)),
            _ => Err("invalid operands for greater than"),
        }
    }
    pub fn leq(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 <= b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a <= b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a <= b)),
            _ => Err("invalid operands for less than or equal"),
        }
    }
    pub fn geq(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 >= b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a >= b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a >= b)),
            _ => Err("invalid operands for greater than or equal"),
        }
    }
    // LOGICAL OPS
    // all the inputs can be any value

    // return the truthiness of a Value
    // non-zero is true, zero is false for int, char, and float
    // non-empty is true, empty is false for lists
    // blocks are always true
    pub fn truthiness_of(val: Value) -> bool {
        match val {
            Value::Int(a) => a != 0,
            Value::Char(a) => a != '\0',
            Value::Float(a) => a != 0.0,
            Value::List(a) => !a.is_empty(),
            Value::Block(_) => true,
            _ => false,
        }
    }
    pub fn lognot(a: Value) -> ValResult {
        match a {
            Value::Bool(a) => Ok(Value::Bool(!a)),
            _ => Ok(Value::Bool(truthiness_of(a))),
        }
    }

    pub fn logand(a: Value, b: Value) -> ValResult {
        match (&a, &b) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a && *b)),
            _ => Ok(Value::Bool(truthiness_of(a) && truthiness_of(b))),
        }
    }
    pub fn logor(a: Value, b: Value) -> ValResult {
        match (&a, &b) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a || *b)),
            _ => Ok(Value::Bool(truthiness_of(a) || truthiness_of(b))),
        }
    }

    // LIST OPS
    pub fn listcat(a: Value, b: Value) -> ValResult {
        // 2 non-lists makes a list
        // a list and a non-list appends the non-list to the list
        // a list and a list concatenates the lists
        match (&a, &b) {
            (Value::List(a), Value::List(b)) => {
                let mut new_list = a.clone();
                new_list.append(&mut b.clone());
                Ok(Value::List(new_list))
            }
            (Value::List(a), b) => {
                let mut a = a.clone();
                a.push(b.clone());
                Ok(Value::List(a))
            }
            (_, Value::List(b)) => {
                let mut b = b.clone();
                b.push(a);
                Ok(Value::List(b))
            }
            _ => Ok(Value::List(vec![a, b])),
        }
    }
    pub fn listslice(list: Value, start: Value, end: Value) -> ValResult {
        match (&list, &start, &end) {
            (Value::List(list), Value::Int(start), Value::Int(end))
            | (Value::Int(end), Value::Int(start), Value::List(list)) => {
                let new_list = list.as_slice()[*start as usize..*end as usize].to_vec();
                Ok(Value::List(new_list))
            }
            _ => Err("invalid operands for list slice"),
        }
    }
    pub fn listindex(list: Value, index: Value) -> ValResult {
        match (&list, &index) {
            (Value::List(list), Value::Int(index)) | (Value::Int(index), Value::List(list)) => {
                if index < &0 {
                    return Err("list index out of bounds");
                }
                match list.get(*index as usize) {
                    Some(val) => Ok(val.clone()),
                    None => Err("list index out of bounds"),
                }
            }
            _ => Err("invalid operands for list index"),
        }
    }
    pub fn listlen(list: Value) -> ValResult {
        match list {
            Value::List(list) => Ok(Value::Int(list.len() as i64)),
            _ => Err("invalid operands for list length"),
        }
    }
    // ld implemented in call_instr

    // CONTROL FLOW
    pub fn exec(deque: &mut VecDeque<Value>, place: Place) {
        let block = match place {
            Place::Left => deque.pop_front().unwrap(),
            Place::Right => deque.pop_back().unwrap(),
        };
        if let Value::Block(block) = block {
            for exec in block {
                match exec {
                    Exec::Left(op) => match op {
                        Op::Literal(lit) => {
                            deque.push_front(lit);
                        }
                        Op::Instruction(instruction) => call_instr(deque, instruction, Place::Left),
                    },
                    Exec::Right(op) => match op {
                        Op::Literal(lit) => {
                            deque.push_back(lit);
                        }
                        Op::Instruction(instruction) => {
                            call_instr(deque, instruction, Place::Right)
                        }
                    },
                }
            }
        }
    }

    // IO
    pub fn il(deque: &mut VecDeque<Value>, place: Place) {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim().to_string();
                // turn input into a Literal::List(Literal::Char)
                let mut input_list = Vec::new();
                for c in input.chars() {
                    input_list.push(Value::Char(c));
                }
                let input_list = Value::List(input_list);
                match place {
                    Place::Left => deque.push_front(input_list),
                    Place::Right => deque.push_back(input_list),
                }
            }
            Err(_) => {
                println!("error reading input");
                std::process::exit(1);
            }
        }
    }
    pub fn ia(deque: &mut VecDeque<Value>, place: Place) {
        // reads everything from stdin and puts it into a list of chars
        let mut input = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_to_string(&mut input) {
            Ok(_) => {
                let input = input.trim().to_string();
                // turn input into a Literal::List(Literal::Char)
                let mut input_list = Vec::new();
                for c in input.chars() {
                    input_list.push(Value::Char(c));
                }
                let input_list = Value::List(input_list);
                match place {
                    Place::Left => deque.push_front(input_list),
                    Place::Right => deque.push_back(input_list),
                }
            }
            Err(_) => {
                println!("error reading input");
                std::process::exit(1);
            }
        }
    }

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
            "len" => len(deque, place),
            "swap" => swap(deque, place),
            // CASTINGS
            "toInt" => unary(deque, place, cast_to_int, true),
            "toFloat" => unary(deque, place, cast_to_float, true),
            "toChar" => unary(deque, place, cast_to_char, true),
            "toBool" => unary(deque, place, cast_to_bool, true),
            // INT/FLOAT OPS
            "+" => binary(deque, place, add, true),
            "-" => binary(deque, place, sub, true),
            "*" => binary(deque, place, mult, true),
            "/" => binary(deque, place, intdiv, true),
            "//" => binary(deque, place, floatdiv, true),
            "%" => binary(deque, place, modulo, true),
            "exp" => binary(deque, place, exp, true),
            "log" => binary(deque, place, log, true),
            "--" => unary(deque, place, neg, true),
            "&" => binary(deque, place, bitand, true),
            "|" => binary(deque, place, bitor, true),
            "^" => binary(deque, place, bitxor, true),
            "n" => unary(deque, place, bitnot, true),
            // COMPARISON OPS
            "==" => binary(deque, place, eq, true),
            "!=" => binary(deque, place, neq, true),
            "<" => binary(deque, place, lt, true),
            ">" => binary(deque, place, gt, true),
            "<=" => binary(deque, place, leq, true),
            ">=" => binary(deque, place, geq, true),
            // LOGICAL OPS
            "nn" => unary(deque, place, lognot, true),
            "&&" => binary(deque, place, logand, true),
            "||" => binary(deque, place, logor, true),

            // LIST OPS
            "l+" => binary(deque, place, listcat, true),
            "l/" => ternary(deque, place, listslice, true),
            "li" => binary(deque, place, listindex, true),
            "ll" => unary(deque, place, listlen, true),
            "ld" => {
                let list = match place {
                    Place::Left => deque.pop_front().unwrap(),
                    Place::Right => deque.pop_back().unwrap(),
                };
                match list {
                    Literal::List(list) => {
                        for elem in list.iter() {
                            match place {
                                Place::Left => deque.push_front(elem.clone()),
                                Place::Right => deque.push_back(elem.clone()),
                            }
                        }
                    }
                    _ => println!("ld: expected list"),
                };
            }

            // CONTROL FLOW OPS
            "exec" => exec(deque, place),
            "loop" => {
                // pop a block and run it forever
                let block = match &place {
                    Place::Left => deque.pop_front().unwrap(),
                    Place::Right => deque.pop_back().unwrap(),
                };
                if let Value::Block(block) = block {
                    loop {
                        for exec in block.iter() {
                            match exec {
                                Exec::Left(op) => match op {
                                    Op::Literal(lit) => {
                                        deque.push_front(lit.clone());
                                    }
                                    Op::Instruction(instruction) => {
                                        call_instr(deque, instruction.clone(), Place::Left)
                                    }
                                },
                                Exec::Right(op) => match op {
                                    Op::Literal(lit) => {
                                        deque.push_back(lit.clone());
                                    }
                                    Op::Instruction(instruction) => {
                                        call_instr(deque, instruction.clone(), Place::Right)
                                    }
                                },
                            }
                        }
                    }
                } else {
                    println!("loop: expected block");
                }
            }

            // IO
            "il" => il(deque, place),
            "ia" => ia(deque, place),
            "ol" => ol(deque, place),
            "ow" => ow(deque, place),
            _ => println!("Unknown instruction: {}", instr),
        }
    }
}
