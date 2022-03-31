mod eval_value {
    use parser::par_ast::*;

    pub type Value = Literal;
}

mod eval {
    use eval_value::*;
    use parser::par_ast::*;
    use std::collections::VecDeque;

    pub fn run_ast(ast: Code) {
        let deque: VecDeque<Value> = VecDeque::new();

        for exec in ast {
            match exec {
                Exec::Left => {}
            }
        }
    }
}
