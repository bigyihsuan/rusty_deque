pub mod visit {
    // https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html
    // https://github.com/rust-unofficial/patterns/discussions/236
    use crate::parser::ast::*;

    pub trait Visitor<T> {
        fn visit_code(&mut self, c: &Code) -> T;
        fn visit_exec(&mut self, e: &Exec) -> T;
        fn visit_op(&mut self, o: &Op) -> T;
        fn visit_instruction(&mut self, i: &Instruction) -> T;
        fn visit_literal(&mut self, l: &Literal) -> T;
        fn visit_block(&mut self, b: &Block) -> T;
    }
}
pub mod tree_print {
    use super::visit::*;
    use crate::parser::ast::*;

    pub struct TreePrinter {
        pub indent: usize,
    }
    impl TreePrinter {
        pub fn new() -> TreePrinter {
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
                Literal::Block(b) => self.visit_block(&b),
            }
        }
        fn visit_block(&mut self, b: &Block) -> String {
            let mut s = String::from("{\n");
            self.indent += 1;
            for be in b {
                for _ in 0..self.indent {
                    s += "    "
                }
                match be {
                    BlockElement::EleExec(e) => {
                        s += &String::from(format!("{}\n", &self.visit_exec(&e)));
                    }
                    BlockElement::EleBlock(bl) => {
                        s += &String::from(format!("{}\n", &self.visit_block(&bl)));
                    }
                }
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
