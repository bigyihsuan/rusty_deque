Code: {{!a b! !c}! !{d! !e f!}}!
BlockBegin { @ 0
BlockBegin { @ 1
Bang ! @ 2
Instruction a @ 3
Instruction b @ 4
Bang ! @ 5
Bang ! @ 6
Instruction c @ 7
BlockEnd } @ 8
Bang ! @ 9
Bang ! @ 10
BlockBegin { @ 11
Instruction d @ 12
Bang ! @ 13
Bang ! @ 14
Instruction e @ 15
Instruction f @ 16
Bang ! @ 17
BlockEnd } @ 18
BlockEnd } @ 19
Bang ! @ 20
End  @ 21
code BlockBegin { @ 0
exec before BlockBegin { @ 0
exec right BlockBegin { @ 0
op before BlockBegin { @ 0
op lit BlockBegin { @ 0
lit BlockBegin { @ 0
block BlockBegin { @ 0
block making BlockBegin { @ 0
stack before []
stack after [[]]
[]
block making BlockBegin { @ 1
stack before [[]]
stack after [[], []]
[]
block making Bang ! @ 2
stack before [[], []]
exec before Bang ! @ 2
exec left Bang ! @ 2
op before Instruction a @ 3
op inst Instruction a @ 3
inst Instruction a @ 3
stack after [[], [Exec(Left(Instruction(Instruction { value: "a" })))]]
[]
block making Instruction b @ 4
stack before [[], [Exec(Left(Instruction(Instruction { value: "a" })))]]
exec before Instruction b @ 4
exec right Instruction b @ 4
op before Instruction b @ 4
op inst Instruction b @ 4
inst Instruction b @ 4
stack after [[], [Exec(Left(Instruction(Instruction { value: "a" }))), Exec(Right(Instruction(Instruction { value: "b" })))]]
[]
block making Bang ! @ 6
stack before [[], [Exec(Left(Instruction(Instruction { value: "a" }))), Exec(Right(Instruction(Instruction { value: "b" })))]]
exec before Bang ! @ 6
exec left Bang ! @ 6
op before Instruction c @ 7
op inst Instruction c @ 7
inst Instruction c @ 7
stack after [[], [Exec(Left(Instruction(Instruction { value: "a" }))), Exec(Right(Instruction(Instruction { value: "b" }))), Exec(Left(Instruction(Instruction { value: "c" })))]]
[]
block making BlockEnd } @ 8
stack before [[], [Exec(Left(Instruction(Instruction { value: "a" }))), Exec(Right(Instruction(Instruction { value: "b" }))), Exec(Left(Instruction(Instruction { value: "c" })))]]
stack after [[]]
[Block([Exec(Left(Instruction(Instruction { value: "a" }))), Exec(Right(Instruction(Instruction { value: "b" }))), Exec(Left(Instruction(Instruction { value: "c" })))])]
block making BlockEnd } @ 8
stack before [[]]
stack after []
[Block([Exec(Left(Instruction(Instruction { value: "a" }))), Exec(Right(Instruction(Instruction { value: "b" }))), Exec(Left(Instruction(Instruction { value: "c" })))])]
code Bang ! @ 9
exec before Bang ! @ 9
exec left Bang ! @ 9
op before Bang ! @ 10
op lit Bang ! @ 10
lit Bang ! @ 10
