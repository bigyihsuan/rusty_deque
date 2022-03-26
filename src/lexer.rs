pub mod lex {

    // The type of a token.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TokenType {
        // SINGLE CHARACTERS
        BANG,
        TILDE,
        COMMA,
        LEFT_CURLY,
        RIGHT_CURLY,
        LEFT_SQUARE,
        RIGHT_SQUARE,
        SINGLE_QUOTE,
        DOUBLE_QUOTE,
        // MULTI-CHARACTERs
        TRUE,
        FALSE,
        INT,
        FLOAT,
        CHAR,
        STRING,
        INSTR,
    }

    // A lexical token.
    #[derive(Debug, Clone, PartialEq)]
    pub struct Token {
        pub token_type: TokenType,
        pub lexeme: String,
        pub start: usize, // start of the token in the source
        pub end: usize,   // end of the token in the source
        pub line: usize,  // line number of the token in the source
    }
}
