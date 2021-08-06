Code: {![1, 2, 0.3, [4, '5', "six", 7.8,], 9,] ol!}!
Token { token_type: BlockBegin, string: "{", index: 0 }
Token { token_type: Bang, string: "!", index: 1 }
Token { token_type: ListBegin, string: "[", index: 2 }
Token { token_type: Literal(Int), string: "1", index: 4 }
Token { token_type: ListSep, string: ",", index: 4 }
Token { token_type: Literal(Int), string: "2", index: 7 }
Token { token_type: ListSep, string: ",", index: 7 }
Token { token_type: Literal(Float), string: "0.3", index: 12 }
Token { token_type: ListSep, string: ",", index: 12 }
Token { token_type: ListBegin, string: "[", index: 14 }
Token { token_type: Literal(Int), string: "4", index: 16 }
Token { token_type: ListSep, string: ",", index: 16 }
Token { token_type: Literal(Char), string: "5", index: 20 }
Token { token_type: ListSep, string: ",", index: 21 }
Token { token_type: Literal(String), string: "six", index: 27 }
Token { token_type: ListSep, string: ",", index: 28 }
Token { token_type: Literal(Float), string: "7.8", index: 33 }
Token { token_type: ListSep, string: ",", index: 33 }
Token { token_type: ListEnd, string: "]", index: 34 }
Token { token_type: ListSep, string: ",", index: 35 }
Token { token_type: Literal(Int), string: "9", index: 38 }
Token { token_type: ListSep, string: ",", index: 38 }
Token { token_type: ListEnd, string: "]", index: 39 }
Token { token_type: Instruction, string: "ol", index: 43 }
Token { token_type: Bang, string: "!", index: 43 }
Token { token_type: BlockEnd, string: "}", index: 44 }
Token { token_type: Bang, string: "!", index: 45 }
Token { token_type: End, string: "", index: 47 }
[]
( 
    !100 
    [true, 2, 3.45, '6', {
        !100 
        [false, 7, 8.9, 'a', ]! 
    }, ]! 

)
