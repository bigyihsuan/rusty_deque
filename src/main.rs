mod lexer;
mod parser;

use lexer::*;
use parser::*;

fn main() {
    // let code =
    //     "!{!dup !2 !rot <!} !rot {pop! 1!}! {!dup !1 !- 2! -1! {!in !*}! !rot !for}! !rot !ite";

    // let code = "{![1, 2, 0.3, [4, '5', \"six\", 7.8,], 9,] ol!}!";
    // let mut c = String::from(code);
    // c.push(' ');
    // let tokens = lexer::lex::tokenize(c);
    // println!("Code: {}", String::from(code));
    // for t in tokens {
    //     println!("{:?}", t);
    // }

    // let code2 = vec![
    //     lexer::tok::Token {
    //         token_type: lexer::tok::TokenType::Literal(lexer::tok::LiteralType::Int),
    //         string: String::from("1"),
    //         index: 0,
    //     },
    //     lexer::tok::Token {
    //         token_type: lexer::tok::TokenType::Bang,
    //         string: String::from("!"),
    //         index: 1,
    //     },
    // ];
    // println!("{:?}", parser::par::parse(code2));
    use visit::Visitor;

    let mut tree_printer = visit::TreePrinter {};

    #[rustfmt::skip]
    let tree = vec![
        ast::Exec::Left(
            ast::Op::Literal(
                ast::Literal::Integer(
                    100,
                )
            )
        ),
        ast::Exec::Right(
            ast::Op::Literal(
                ast::Literal::List(
                    vec![
                        Box::new(ast::Literal::Boolean(true)),
                        Box::new(ast::Literal::Integer(2)),
                        Box::new(ast::Literal::Float(3.45)),
                        Box::new(ast::Literal::Character('6')),
                        Box::new(ast::Literal::Block(
                            vec![
                                ast::Exec::Left(
                                    ast::Op::Literal(
                                        ast::Literal::Integer(
                                            100,
                                        )
                                    )
                                ),
                                ast::Exec::Right(
                                    ast::Op::Literal(
                                        ast::Literal::List(
                                            vec![
                                                Box::new(ast::Literal::Boolean(false)),
                                                Box::new(ast::Literal::Integer(7)),
                                                Box::new(ast::Literal::Float(8.9)),
                                                Box::new(ast::Literal::Character('a')),
                                            ]
                                        )
                                    )
                                )
                            ]
                        ))
                    ]
                )
            )
        )
    ];

    println!("{}", tree_printer.visit_code(&tree));
}
