Code: {{a! b! c!}! {d! e! f!}!}!
BlockBegin { @ 0
BlockBegin { @ 1
Instruction a @ 2
Bang ! @ 3
Instruction b @ 4
Bang ! @ 5
Instruction c @ 6
Bang ! @ 7
BlockEnd } @ 8
Bang ! @ 9
BlockBegin { @ 10
Instruction d @ 11
Bang ! @ 12
Instruction e @ 13
Bang ! @ 14
Instruction f @ 15
Bang ! @ 16
BlockEnd } @ 17
Bang ! @ 18
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
[]
[]
[[]]
[]
[[], []]
[]
adding new exec
exec before Instruction a @ 2
exec right Instruction a @ 2
op before Instruction a @ 2
op inst Instruction a @ 2
inst Instruction a @ 2
[[], [EleExec(Right(Instruction(Instruction { value: "a" })))]]
[]
adding new exec
exec before Instruction b @ 4
exec right Instruction b @ 4
op before Instruction b @ 4
op inst Instruction b @ 4
inst Instruction b @ 4
[[], [EleExec(Right(Instruction(Instruction { value: "a" }))), EleExec(Right(Instruction(Instruction { value: "b" })))]]
[]
adding new exec
exec before Instruction c @ 6
exec right Instruction c @ 6
op before Instruction c @ 6
op inst Instruction c @ 6
inst Instruction c @ 6
[[], [EleExec(Right(Instruction(Instruction { value: "a" }))), EleExec(Right(Instruction(Instruction { value: "b" }))), EleExec(Right(Instruction(Instruction { value: "c" })))]]
[]
nested block
[[EleBlock([EleExec(Right(Instruction(Instruction { value: "a" }))), EleExec(Right(Instruction(Instruction { value: "b" }))), EleExec(Right(Instruction(Instruction { value: "c" })))])]]
[]
adding new exec
exec before Bang ! @ 9
exec left Bang ! @ 9
op before BlockBegin { @ 10
op lit BlockBegin { @ 10
lit BlockBegin { @ 10
block BlockBegin { @ 10
[]
[]
[[]]
[]
adding new exec
exec before Instruction d @ 11
exec right Instruction d @ 11
op before Instruction d @ 11
op inst Instruction d @ 11
inst Instruction d @ 11
[[EleExec(Right(Instruction(Instruction { value: "d" })))]]
[]
adding new exec
exec before Instruction e @ 13
exec right Instruction e @ 13
op before Instruction e @ 13
op inst Instruction e @ 13
inst Instruction e @ 13
[[EleExec(Right(Instruction(Instruction { value: "d" }))), EleExec(Right(Instruction(Instruction { value: "e" })))]]
[]
adding new exec
exec before Instruction f @ 15
exec right Instruction f @ 15
op before Instruction f @ 15
op inst Instruction f @ 15
inst Instruction f @ 15
[[EleExec(Right(Instruction(Instruction { value: "d" }))), EleExec(Right(Instruction(Instruction { value: "e" }))), EleExec(Right(Instruction(Instruction { value: "f" })))]]
[]
end of current block
block return [EleExec(Right(Instruction(Instruction { value: "d" }))), EleExec(Right(Instruction(Instruction { value: "e" }))), EleExec(Right(Instruction(Instruction { value: "f" })))]
[[EleBlock([EleExec(Right(Instruction(Instruction { value: "a" }))), EleExec(Right(Instruction(Instruction { value: "b" }))), EleExec(Right(Instruction(Instruction { value: "c" })))]), EleExec(Left(Literal(Block([EleExec(Right(Instruction(Instruction { value: "d" }))), EleExec(Right(Instruction(Instruction { value: "e" }))), EleExec(Right(Instruction(Instruction { value: "f" })))]))))]]
[]
adding new exec
exec before Bang ! @ 18
exec left Bang ! @ 18
op before BlockEnd } @ 19
op lit BlockEnd } @ 19
lit BlockEnd } @ 19
