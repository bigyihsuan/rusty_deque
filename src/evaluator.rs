pub mod eval_value {
    use crate::parser::par_ast::*;

    pub type Value = Literal;

    #[derive(Debug, Clone, Copy)]
    pub enum Place {
        Left,
        Right,
    }
}

pub mod eval_instr {
    use super::{eval::*, eval_value::*};
    use crate::parser::par_ast::*;
    use std::{
        collections::VecDeque,
        io::{self, Read},
        str::FromStr,
    };

    type ValResult = Result<Value, String>;
    type Nilary = fn() -> ValResult;
    type Unary = fn(a: Value) -> ValResult;
    type Binary = fn(a: Value, b: Value) -> ValResult;
    type Ternary = fn(a: Value, b: Value, c: Value) -> ValResult;
    // type Quaternary = fn(a: Value, b: Value, c: Value, d: Value) -> ValResult;
    pub type FnResult = Result<(), String>;

    // bool is to push the result back to the stack
    pub fn nilary(
        deque: &mut VecDeque<Value>,
        place: Place,
        func: Nilary,
        push_result: bool,
    ) -> FnResult {
        let result = func()?;
        match place {
            Place::Left => {
                if push_result {
                    deque.push_front(result);
                }
            }
            Place::Right => {
                if push_result {
                    deque.push_back(result);
                }
            }
        }
        Ok(())
    }
    pub fn unary(
        deque: &mut VecDeque<Value>,
        place: Place,
        func: Unary,
        push_result: bool,
    ) -> FnResult {
        match place {
            Place::Left => {
                let val = deque.pop_front().ok_or_else(|| "tried to pop empty deque");
                match val {
                    Ok(val) => {
                        let result = func(val)?;
                        if push_result {
                            deque.push_front(result);
                        }
                        Ok(())
                    }
                    Err(err) => Err(err.to_string()),
                }
            }
            Place::Right => {
                let val = deque.pop_back().ok_or_else(|| "tried to pop empty deque");
                match val {
                    Ok(val) => {
                        let result = func(val)?;
                        if push_result {
                            deque.push_back(result);
                        }
                        Ok(())
                    }
                    Err(err) => Err(err.to_string()),
                }
            }
        }
    }

    pub fn binary(
        deque: &mut VecDeque<Value>,
        place: Place,
        func: Binary,
        push_result: bool,
    ) -> FnResult {
        match place {
            Place::Left => {
                let val_a = deque.pop_front().ok_or_else(|| "tried to pop empty deque");
                let val_b = deque.pop_front().ok_or_else(|| "tried to pop empty deque");
                match (val_a, val_b) {
                    (Ok(val_a), Ok(val_b)) => {
                        let result = func(val_a, val_b)?;
                        if push_result {
                            deque.push_front(result);
                        }
                        Ok(())
                    }
                    (Err(err), _) => Err(err.to_string()),
                    (_, Err(err)) => Err(err.to_string()),
                }
            }
            Place::Right => {
                let val_a = deque.pop_back().ok_or_else(|| "tried to pop empty deque");
                let val_b = deque.pop_back().ok_or_else(|| "tried to pop empty deque");
                match (val_a, val_b) {
                    (Ok(val_a), Ok(val_b)) => {
                        let result = func(val_a, val_b)?;
                        if push_result {
                            deque.push_front(result);
                        }
                        Ok(())
                    }
                    (Err(err), _) => Err(err.to_string()),
                    (_, Err(err)) => Err(err.to_string()),
                }
            }
        }
    }
    pub fn ternary(
        deque: &mut VecDeque<Value>,
        place: Place,
        func: Ternary,
        push_result: bool,
    ) -> FnResult {
        match place {
            Place::Left => {
                let val_a = deque.pop_front().ok_or_else(|| "tried to pop empty deque");
                let val_b = deque.pop_front().ok_or_else(|| "tried to pop empty deque");
                let val_c = deque.pop_front().ok_or_else(|| "tried to pop empty deque");
                match (val_a, val_b, val_c) {
                    (Ok(val_a), Ok(val_b), Ok(val_c)) => {
                        let result = func(val_a, val_b, val_c)?;
                        if push_result {
                            deque.push_front(result);
                        }
                        Ok(())
                    }
                    (Err(err), _, _) => Err(err.to_string()),
                    (_, Err(err), _) => Err(err.to_string()),
                    (_, _, Err(err)) => Err(err.to_string()),
                }
            }
            Place::Right => {
                let val_a = deque.pop_back().ok_or_else(|| "tried to pop empty deque");
                let val_b = deque.pop_back().ok_or_else(|| "tried to pop empty deque");
                let val_c = deque.pop_back().ok_or_else(|| "tried to pop empty deque");
                match (val_a, val_b, val_c) {
                    (Ok(val_a), Ok(val_b), Ok(val_c)) => {
                        let result = func(val_a, val_b, val_c)?;
                        if push_result {
                            deque.push_front(result);
                        }
                        Ok(())
                    }
                    (Err(err), _, _) => Err(err.to_string()),
                    (_, Err(err), _) => Err(err.to_string()),
                    (_, _, Err(err)) => Err(err.to_string()),
                }
            }
        }
    }

    // pub fn quaternary(
    //     deque: &mut VecDeque<Value>,
    //     place: Place,
    //     func: Quaternary,
    //     push_result: bool,
    // ) {
    //     match place {
    //         Place::Left => {
    //             let val_a = deque.pop_front().unwrap();
    //             let val_b = deque.pop_front().unwrap();
    //             let val_c = deque.pop_front().unwrap();
    //             let val_d = deque.pop_front().unwrap();
    //             let result = func(val_a, val_b, val_c, val_d);
    //             if push_result {
    //                 match result {
    //                     Ok(v) => match v {
    //                         Value::None => {}
    //                         _ => deque.push_front(v),
    //                     },
    //                     Err(e) => println!("{}", e),
    //                 }
    //             }
    //         }
    //         Place::Right => {
    //             let val_a = deque.pop_back().unwrap();
    //             let val_b = deque.pop_back().unwrap();
    //             let val_c = deque.pop_back().unwrap();
    //             let val_d = deque.pop_back().unwrap();
    //             let result = func(val_a, val_b, val_c, val_d);
    //             if push_result {
    //                 match result {
    //                     Ok(v) => match v {
    //                         Value::None => {}
    //                         _ => deque.push_back(v),
    //                     },
    //                     Err(e) => println!("{}", e),
    //                 }
    //             }
    //         }
    //     }
    // }

    // DEQUE OPS
    pub fn clear(deque: &mut VecDeque<Value>, _place: Place) -> FnResult {
        deque.clear();
        Ok(())
    }

    pub fn pop(deque: &mut VecDeque<Value>, place: Place) -> FnResult {
        match place {
            Place::Left => {
                let result = deque.pop_front();
                match result {
                    Some(_) => Ok(()),
                    None => Err("tried to pop empty deque".to_string()),
                }
            }
            Place::Right => {
                let result = deque.pop_back();
                match result {
                    Some(_) => Ok(()),
                    None => Err("tried to pop empty deque".to_string()),
                }
            }
        }
    }

    pub fn dup(deque: &mut VecDeque<Value>, place: Place) -> FnResult {
        match place {
            Place::Left => {
                let val = deque.front();
                match val {
                    Some(v) => {
                        let dupped = v.clone();
                        deque.push_front(dupped);
                        Ok(())
                    }
                    None => Err("tried to dup empty deque".to_string()),
                }
            }
            Place::Right => {
                let val = deque.back();
                match val {
                    Some(v) => {
                        let dupped = v.clone();
                        deque.push_back(dupped);
                        Ok(())
                    }
                    None => Err("tried to dup empty deque".to_string()),
                }
            }
        }
    }

    pub fn rot(deque: &mut VecDeque<Value>, place: Place) -> FnResult {
        match place {
            Place::Left => {
                deque.rotate_left(1);
            }
            Place::Right => {
                deque.rotate_right(1);
            }
        }
        Ok(())
    }

    pub fn over(deque: &mut VecDeque<Value>, place: Place) -> FnResult {
        let mut iter = deque.iter();
        match place {
            Place::Left => {
                iter.next();
                let ele = iter
                    .next()
                    .ok_or_else(|| "tried to over deque with less than 2 items")
                    .and_then(|v| {
                        if let Value::None = v {
                            Err("tried to over deque with less than 2 items")
                        } else {
                            Ok(v)
                        }
                    });
                match ele {
                    Ok(v) => {
                        let dupped = v.clone();
                        deque.push_back(dupped);
                        Ok(())
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            Place::Right => {
                iter.next_back();
                let ele = iter
                    .next_back()
                    .ok_or_else(|| "tried to over deque with less than 2 items")
                    .and_then(|v| {
                        if let Value::None = v {
                            Err("tried to over deque with less than 2 items")
                        } else {
                            Ok(v)
                        }
                    });
                match ele {
                    Ok(v) => {
                        let dupped = v.clone();
                        deque.push_front(dupped);
                        Ok(())
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
        }
    }

    pub fn swap(deque: &mut VecDeque<Value>, place: Place) -> FnResult {
        match place {
            Place::Left => {
                let val_a = deque.pop_front();
                let val_b = deque.pop_front();
                match (val_a, val_b) {
                    (Some(val_a), Some(val_b)) => {
                        deque.push_front(val_a);
                        deque.push_front(val_b);
                        Ok(())
                    }
                    (Some(_), None) => Err("tried to swap on deque with 1 item".to_string()),
                    (None, Some(_)) => Err("tried to swap on deque with 1 item".to_string()),
                    (None, None) => Ok(()),
                }
            }
            Place::Right => {
                let val_a = deque.pop_back();
                let val_b = deque.pop_back();
                match (val_a, val_b) {
                    (Some(val_a), Some(val_b)) => {
                        deque.push_back(val_a);
                        deque.push_back(val_b);
                        Ok(())
                    }
                    (Some(_), None) => Err("tried to swap on deque with 1 item".to_string()),
                    (None, Some(_)) => Err("tried to swap on deque with 1 item".to_string()),
                    (None, None) => Ok(()),
                }
            }
        }
    }

    pub fn len(deque: &mut VecDeque<Value>, place: Place) -> FnResult {
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
        Ok(())
    }
    // CASTINGS

    // attempts to cast a Value::List of chars to a type T
    // it should only attempt to cast if all the elements in the list are Value::Char
    // Returns a Result, which is Ok(T) if the cast is successful, and Err(String) if it is not
    pub fn try_cast_list_to<T>(list: Vec<Value>) -> Result<T, String>
    where
        T: FromStr,
    {
        let mut chars: Vec<char> = Vec::new();
        for ele in list {
            match ele {
                Value::Char(c) => chars.push(c),
                _ => return Err(format!("{} is not a char", ele.to_string())),
            }
        }
        let str_val: String = chars.iter().collect();
        // cast the str to the desired type
        match str_val.parse::<T>() {
            Ok(v) => Ok(v),
            Err(_) => Err("Could not cast to desired type".to_string()),
        }
    }

    pub fn cast_to_int(val: Value) -> ValResult {
        match val {
            Value::Int(i) => Ok(Value::Int(i)),
            Value::Float(f) => Ok(Value::Int(f as i64)),
            Value::Bool(b) => Ok(Value::Int(if b { 1 } else { 0 })),
            Value::Char(c) => Ok(Value::Int(c as i64)),
            Value::List(l) => {
                let result = try_cast_list_to::<i64>(l);
                match result {
                    Ok(v) => Ok(Value::Int(v)),
                    Err(_) => Err("Could not cast to int".to_string()),
                }
            }
            _ => Err("Cannot cast to int".to_string()),
        }
    }
    pub fn cast_to_float(val: Value) -> ValResult {
        match val {
            Value::Int(i) => Ok(Value::Float(i as f64)),
            Value::Float(f) => Ok(Value::Float(f)),
            Value::Bool(b) => Ok(Value::Float(if b { 1.0 } else { 0.0 })),
            Value::Char(c) => Ok(Value::Float(c as i64 as f64)),
            Value::List(l) => {
                let result = try_cast_list_to::<f64>(l);
                match result {
                    Ok(v) => Ok(Value::Float(v)),
                    Err(_) => Err("Could not cast to float".to_string()),
                }
            }
            _ => Err("Cannot cast to float".to_string()),
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
            Value::None => Ok(Value::Bool(false)), // this shouldn't happen, but if it doesn None is falsy
        }
    }
    pub fn cast_to_char(val: Value) -> ValResult {
        match val {
            Value::Int(i) => Ok(Value::Char(i as u8 as char)),
            Value::Float(f) => Ok(Value::Char(f as u8 as char)),
            Value::Bool(b) => Ok(Value::Char(if b { '1' } else { '0' })),
            Value::Char(c) => Ok(Value::Char(c)),
            _ => Err("Cannot cast to char".to_string()),
        }
    }

    // INT/FLOAT OPS
    pub fn add(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            _ => Err("invalid operands for addition".to_string()),
        }
    }
    pub fn sub(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            _ => Err("invalid operands for subtraction".to_string()),
        }
    }
    pub fn mult(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            _ => Err("invalid operands for multiplication".to_string()),
        }
    }
    pub fn intdiv(a: Value, b: Value) -> ValResult {
        if b == Value::Int(0) || b == Value::Float(0.0) {
            return Err("integer division by zero".to_string());
        }
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 / b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            _ => Err("invalid operands for integer division".to_string()),
        }
    }
    pub fn floatdiv(a: Value, b: Value) -> ValResult {
        if b == Value::Int(0) || b == Value::Float(0.0) {
            return Err("float division by zero".to_string());
        }
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Float(a as f64 / b as f64)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 / b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            _ => Err("invalid operands for float division".to_string()),
        }
    }
    pub fn modulo(a: Value, b: Value) -> ValResult {
        if b == Value::Int(0) || b == Value::Float(0.0) {
            return Err("modulo by zero".to_string());
        }
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a % b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 % b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a % b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
            _ => Err("invalid operands for modulo".to_string()),
        }
    }
    pub fn exp(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Float((a as f64).powf(b as f64))),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f64).powf(b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.powf(b as f64))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(b))),
            _ => Err("invalid operands for exponentiation".to_string()),
        }
    }
    pub fn log(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Float((a as f64).log(b as f64))),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f64).log(b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.log(b as f64))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.log(b))),
            _ => Err("invalid operands for logarithm".to_string()),
        }
    }
    pub fn neg(a: Value) -> ValResult {
        match a {
            Value::Int(a) => Ok(Value::Int(-a)),
            Value::Float(a) => Ok(Value::Float(-a)),
            _ => Err("invalid operand for negation".to_string()),
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
            _ => Err("invalid operands for bitwise AND".to_string()),
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
            _ => Err("invalid operands for bitwise OR".to_string()),
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
            _ => Err("invalid operands for bitwise XOR".to_string()),
        }
    }
    pub fn bitnot(a: Value) -> ValResult {
        match a {
            Value::Int(a) => Ok(Value::Int(!a)),
            Value::Float(a) => Ok(Value::Int(!(a.to_bits()) as i64)),
            _ => Err("invalid operand for bitwise NOT".to_string()),
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
            _ => Err("invalid operands for equality".to_string()),
        }
    }
    pub fn neq(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a != b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 != b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a != b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a != b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a != b)),
            _ => Err("invalid operands for inequality".to_string()),
        }
    }
    pub fn lt(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) < b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a < b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a < b)),
            _ => Err("invalid operands for less than".to_string()),
        }
    }
    pub fn gt(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 > b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a > b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a > b)),
            _ => Err("invalid operands for greater than".to_string()),
        }
    }
    pub fn leq(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 <= b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a <= b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a <= b)),
            _ => Err("invalid operands for less than or equal".to_string()),
        }
    }
    pub fn geq(a: Value, b: Value) -> ValResult {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 >= b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a >= b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a >= b)),
            _ => Err("invalid operands for greater than or equal".to_string()),
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
            Value::Bool(a) => a,
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
    pub fn listjoin(list: Value, joiner: Value) -> ValResult {
        match list {
            Value::List(list) => {
                let mut new_list = Vec::new();
                for val in list {
                    new_list.push(val);
                    new_list.push(joiner.clone());
                }
                new_list.pop(); // remove the final extra joiner
                Ok(Value::List(new_list))
            }
            _ => Err("listjoin: first argument must be a list".to_string()),
        }
    }
    pub fn listslice(list: Value, start: Value, end: Value) -> ValResult {
        match (&list, &start, &end) {
            (Value::List(list), Value::Int(start), Value::Int(end))
            | (Value::Int(end), Value::Int(start), Value::List(list)) => {
                let new_list = list.as_slice()[*start as usize..*end as usize].to_vec();
                Ok(Value::List(new_list))
            }
            _ => Err("invalid operands for list slice".to_string()),
        }
    }
    pub fn listindex(list: Value, index: Value) -> ValResult {
        match (&list, &index) {
            (Value::List(list), Value::Int(index)) | (Value::Int(index), Value::List(list)) => {
                if index < &0 {
                    return Err("list index out of bounds".to_string());
                }
                match list.get(*index as usize) {
                    Some(val) => Ok(val.clone()),
                    None => Err("list index out of bounds".to_string()),
                }
            }
            _ => Err("invalid operands for list index".to_string()),
        }
    }
    pub fn listlen(list: Value) -> ValResult {
        match list {
            Value::List(list) => Ok(Value::Int(list.len() as i64)),
            _ => Err("invalid operands for list length".to_string()),
        }
    }
    // lb implemented in call_instr
    // ld implemented in call_instr

    // LIST FUNCTIONS
    pub fn map(list: Value, block: Value) -> ValResult {
        match (list, &block) {
            (Value::List(list), Value::Block(_)) => {
                let mut new_list = Vec::new();
                let mut temp_deque: VecDeque<Value> = VecDeque::new();
                for val in list {
                    temp_deque.push_front(val.clone());
                    exec_block(&mut temp_deque, &block)?;
                    new_list.push(temp_deque.pop_front().unwrap());
                }
                Ok(Value::List(new_list))
            }
            _ => Err("invalid operands for map".to_string()),
        }
    }
    pub fn filter(list: Value, block: Value) -> ValResult {
        match (list, &block) {
            (Value::List(list), Value::Block(_)) => {
                let mut new_list = Vec::new();
                let mut temp_deque: VecDeque<Value> = VecDeque::new();
                for val in list {
                    temp_deque.push_front(val.clone());
                    exec_block(&mut temp_deque, &block)?;
                    let result = temp_deque.pop_front().unwrap();
                    if let Value::Bool(true) = result {
                        new_list.push(val.clone());
                    }
                }
                Ok(Value::List(new_list))
            }
            _ => Err("invalid operands for filter".to_string()),
        }
    }
    pub fn reduce(list: Value, accumulator: Value, block: Value) -> ValResult {
        match (list, &accumulator, &block) {
            (Value::List(list), _, Value::Block(_)) => {
                let mut temp_deque: VecDeque<Value> = VecDeque::new();
                let mut result = accumulator;
                for val in list {
                    temp_deque.push_front(val.clone());
                    temp_deque.push_back(result.clone());
                    exec_block(&mut temp_deque, &block)?;
                    result = temp_deque.pop_front().unwrap();
                }
                Ok(result)
            }
            _ => Err("invalid operands for reduce".to_string()),
        }
    }

    // CONTROL FLOW
    pub fn exec(deque: &mut VecDeque<Value>, place: Place) -> FnResult {
        let block = match place {
            Place::Left => deque.pop_front(),
            Place::Right => deque.pop_back(),
        };
        if let Some(block) = block {
            if let Value::Block(block) = block {
                for exec in block {
                    match exec {
                        Exec::Left(op) => match op {
                            Op::Literal(lit) => {
                                deque.push_front(lit);
                            }
                            Op::Instruction(instruction) => {
                                let result = call_instr(deque, instruction, Place::Left);
                                if let Err(err) = result {
                                    return Err(err);
                                }
                            }
                        },
                        Exec::Right(op) => match op {
                            Op::Literal(lit) => {
                                deque.push_back(lit);
                            }
                            Op::Instruction(instruction) => {
                                let result = call_instr(deque, instruction, Place::Right);
                                if let Err(err) = result {
                                    return Err(err);
                                }
                            }
                        },
                    }
                }
            }
        } else {
            return Err("Failed to pop block".to_string());
        }
        Ok(())
    }

    // utility function to execute a block for the control flow instructions
    pub fn exec_block(deque: &mut VecDeque<Value>, block: &Value) -> FnResult {
        if let Value::Block(block) = block {
            for exec in block {
                // println!("{:?}", deque);
                // println!("executing: {:?}", exec);
                match exec {
                    Exec::Left(op) => match op {
                        Op::Literal(lit) => {
                            deque.push_front(lit.clone());
                        }
                        Op::Instruction(instruction) => {
                            let result = call_instr(deque, instruction.clone(), Place::Left);
                            if let Err(err) = result {
                                return Err(err);
                            }
                        }
                    },
                    Exec::Right(op) => match op {
                        Op::Literal(lit) => {
                            deque.push_back(lit.clone());
                        }
                        Op::Instruction(instruction) => {
                            let result = call_instr(deque, instruction.clone(), Place::Right);
                            if let Err(err) = result {
                                return Err(err);
                            }
                        }
                    },
                }
            }
        }
        Ok(())
    }

    pub fn loop_instr(deque: &mut VecDeque<Value>, place: Place) -> FnResult {
        // pop a block and run it forever
        let block = match &place {
            Place::Left => deque.pop_front(),
            Place::Right => deque.pop_back(),
        };
        if let Some(block) = block {
            if let Value::Block(_) = &block {
                loop {
                    exec_block(deque, &block)?;
                }
            } else {
                return Err("loop: expected block".to_string());
            }
        } else {
            return Err("loop: failed to pop block".to_string());
        }
    }

    pub fn range(deque: &mut VecDeque<Value>, place: Place) -> FnResult {
        // pop 4 times:
        // lower bound, upper bound, increment size, loop body block
        // run the loop body block for each value in the range
        let (lower, upper, inc, body) = match &place {
            Place::Left => (
                deque.pop_front(),
                deque.pop_front(),
                deque.pop_front(),
                deque.pop_front(),
            ),
            Place::Right => (
                deque.pop_back(),
                deque.pop_back(),
                deque.pop_back(),
                deque.pop_back(),
            ),
        };
        match (lower, upper, inc, body) {
            (Some(lower), Some(upper), Some(inc), Some(body)) => {
                if let (Value::Int(lower), Value::Int(upper), Value::Int(inc), Value::Block(_)) =
                    (lower.clone(), upper.clone(), inc.clone(), &body)
                {
                    for i in (lower..upper).step_by(inc as usize) {
                        match place {
                            Place::Left => deque.push_front(Value::Int(i as i64)),
                            Place::Right => deque.push_back(Value::Int(i as i64)),
                        }
                        let result = exec_block(deque, &body);
                        if let Err(err) = result {
                            return Err(err);
                        }
                    }
                    Ok(())
                } else {
                    Err(format!(
                        "range: expected start, end, step, and block; instead got {:?}",
                        (
                            &lower.to_string(),
                            &upper.to_string(),
                            &inc.to_string(),
                            body
                        )
                    ))
                }
            }
            _ => Err("range: expected start, end, step, and block".to_string()),
        }
    }

    pub fn while_instr(deque: &mut VecDeque<Value>, place: Place) -> FnResult {
        // pop 2 times:
        // condition block, loop body block
        let (condition, loop_body) = match place {
            Place::Left => (deque.pop_front().unwrap(), deque.pop_front().unwrap()),
            Place::Right => (deque.pop_back().unwrap(), deque.pop_back().unwrap()),
        };
        if let Value::Block(_) = condition {
            if let Value::Block(_) = loop_body {
                loop {
                    let result = exec_block(deque, &condition);
                    if let Err(err) = result {
                        return Err(err);
                    }
                    let top = match place {
                        Place::Left => deque.pop_front(),
                        Place::Right => deque.pop_back(),
                    };
                    if let Some(top) = top {
                        // exit the loop if the condition is false
                        let truth = truthiness_of(top.clone());
                        if !truth {
                            break;
                        }
                        let result = exec_block(deque, &loop_body);
                        if let Err(err) = result {
                            return Err(err);
                        }
                    } else {
                        return Err("while: failed to get condition result".to_string());
                    }
                }
                Ok(())
            } else {
                Err("while: expected loop body block".to_string())
            }
        } else {
            Err("while: expected condition block".to_string())
        }
    }

    pub fn ite(deque: &mut VecDeque<Value>, place: Place) -> FnResult {
        // pop 3 blocks: condition, true, false
        let (condition, true_block, false_block) = match place {
            Place::Left => (deque.pop_front(), deque.pop_front(), deque.pop_front()),
            Place::Right => (deque.pop_back(), deque.pop_back(), deque.pop_back()),
        };
        match (condition, true_block, false_block) {
            (Some(condition), Some(true_block), Some(false_block)) => {
                // exec condition
                let result = exec_block(deque, &condition);
                if let Err(err) = result {
                    return Err(err);
                }
                // check the truthiness of the top of the stack
                let truth = match place {
                    Place::Left => deque.pop_front(),
                    Place::Right => deque.pop_back(),
                };
                if let Some(truth) = truth {
                    let truth = truthiness_of(truth.clone());
                    if truth {
                        let result = exec_block(deque, &true_block);
                        if let Err(err) = result {
                            return Err(err);
                        }
                    } else {
                        let result = exec_block(deque, &false_block);
                        if let Err(err) = result {
                            return Err(err);
                        }
                    }
                    Ok(())
                } else {
                    return Err("ite: failed to get condition result".to_string());
                }
            }
            _ => Err("ite: expected condition, true, and false blocks".to_string()),
        }
    }

    // IO
    pub fn il() -> ValResult {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim().to_string();
                // turn input into a Literal::List(Literal::Char)
                let mut input_list = Vec::new();
                for c in input.chars() {
                    input_list.push(Value::Char(c));
                }
                Ok(Value::List(input_list))
            }
            Err(_) => Err("il: error reading from stdin".to_string()),
        }
    }
    pub fn ia() -> ValResult {
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
                Ok(Value::List(input_list))
            }
            Err(_) => Err("ia: error reading from stdin".to_string()),
        }
    }

    pub fn ol(literal: Value) -> ValResult {
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
        Ok(Value::None)
    }

    // pretty much the same as ol, consider consolidating
    pub fn ow(literal: Value) -> ValResult {
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
        Ok(Value::None)
    }
}

pub mod eval {
    use super::eval_value::*;
    use crate::evaluator::eval_instr::*;
    use crate::parser::par_ast::*;

    use std::collections::VecDeque;

    pub fn run_ast(deque: Option<VecDeque<Value>>, ast: Code) -> Result<VecDeque<Value>, String> {
        let temp: VecDeque<Value> = VecDeque::new();
        let d: &mut VecDeque<Value> = &mut deque.unwrap_or(temp);

        for exec in ast {
            match exec {
                Exec::Left(op) => match op {
                    Op::Literal(lit) => {
                        d.push_front(lit);
                    }
                    Op::Instruction(instruction) => {
                        let result = call_instr(d, instruction, Place::Left);
                        if let Err(err) = result {
                            return Err(err);
                        }
                    }
                },
                Exec::Right(op) => match op {
                    Op::Literal(lit) => {
                        d.push_back(lit);
                    }
                    Op::Instruction(instruction) => {
                        let result = call_instr(d, instruction, Place::Right);
                        if let Err(err) = result {
                            return Err(err);
                        }
                    }
                },
            }
        }
        Ok(d.to_owned())
    }

    pub fn call_instr(deque: &mut VecDeque<Value>, instr: String, place: Place) -> FnResult {
        match instr.as_str() {
            // DEQUE OPS
            "clear" => clear(deque, place),
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
            "=" => binary(deque, place, eq, true),
            "ne" => binary(deque, place, neq, true),
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
            "lj" => binary(deque, place, listjoin, true),
            "l/" => ternary(deque, place, listslice, true),
            "li" => binary(deque, place, listindex, true),
            "ll" => unary(deque, place, listlen, true),
            "lb" => {
                let lit = match place {
                    Place::Left => deque.pop_front(),
                    Place::Right => deque.pop_back(),
                };
                if let Some(lit) = lit {
                    match lit {
                        Literal::Int(element_count) => {
                            let mut list: Vec<Value> = Vec::new();
                            for _ in 0..element_count {
                                let val = match place {
                                    Place::Left => deque.pop_front(),
                                    Place::Right => deque.pop_back(),
                                };
                                match val {
                                    Some(val) => list.push(val),
                                    None => {
                                        return Err("Not enough elements in deque to fill list."
                                            .to_string());
                                    }
                                }
                            }
                            match place {
                                Place::Left => deque.push_front(Literal::List(list)),
                                Place::Right => deque.push_back(Literal::List(list)),
                            }
                        }
                        _ => return Err("lb: expected int".to_string()),
                    }
                    Ok(())
                } else {
                    Err("lb: expected int".to_string())
                }
            }
            "ld" => {
                let list = match place {
                    Place::Left => deque.pop_front(),
                    Place::Right => deque.pop_back(),
                };
                if let Some(list) = list {
                    match list {
                        Literal::List(list) => {
                            for elem in list.iter() {
                                match place {
                                    Place::Left => deque.push_front(elem.clone()),
                                    Place::Right => deque.push_back(elem.clone()),
                                }
                            }
                        }
                        _ => return Err("ld: expected list".to_string()),
                    };
                    Ok(())
                } else {
                    Err("ld: expected list".to_string())
                }
            }
            // LIST FUNCTIONS
            "map" => binary(deque, place, map, true),
            "filter" => binary(deque, place, filter, true),
            "reduce" => ternary(deque, place, reduce, true),

            // CONTROL FLOW OPS
            "exec" => exec(deque, place),
            "loop" => loop_instr(deque, place),
            "range" => range(deque, place),
            "while" => while_instr(deque, place),
            "ite" => ite(deque, place),

            // IO
            "il" => nilary(deque, place, il, true),
            "ia" => nilary(deque, place, ia, true),
            "ol" => unary(deque, place, ol, false),
            "ow" => unary(deque, place, ow, false),
            _ => Err(format!("Unknown instruction: {}", instr)),
        }
    }
}
