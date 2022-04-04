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

    type ValResult = Result<Value, &'static str>;
    type Unary = fn(a: Value) -> ValResult;
    type Binary = fn(a: Value, b: Value) -> ValResult;

    // bool is to push the result back to the stack
    pub fn unary(deque: &mut VecDeque<Value>, place: Place, func: Unary, push_result: bool) {
        match place {
            Place::Left => {
                let val = deque.pop_front().unwrap();
                let result = func(val);
                if push_result {
                    match result {
                        Ok(v) => deque.push_front(v),
                        Err(e) => panic!("{}", e),
                    }
                }
            }
            Place::Right => {
                let val = deque.pop_back().unwrap();
                let result = func(val);
                if push_result {
                    match result {
                        Ok(v) => deque.push_back(v),
                        Err(e) => panic!("{}", e),
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
                        Err(e) => panic!("{}", e),
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
                        Err(e) => panic!("{}", e),
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

            // IO
            "ol" => ol(deque, place),
            "ow" => ow(deque, place),
            _ => panic!("Unknown instruction: {}", instr),
        }
    }
}
