; Load data
LDI rF 9 ; elem 0
LDI r1 0
STR r1 rf
INC r1
LDI rF 7 ; elem 1
LDI r1 1
STR r1 rf
INC r1
LDI rF 8 ; elem 2
LDI r1 2
STR r1 rf
INC r1
LDI rF 2 ; elem 3
LDI r1 3
STR r1 rf
INC r1
LDI rF 3 ; elem 4
LDI r1 4
STR r1 rf
INC r1
LDI rF 3 ; elem 5
LDI r1 5
STR r1 rf
INC r1
LDI rF 4 ; elem 6
LDI r1 6
STR r1 rf
INC r1
LDI rF 1 ; elem 7
LDI r1 7
STR r1 rf
INC r1
LDI rF 200 ; elem 8
LDI r1 8
STR r1 rf
INC r1
LDI rF 6 ; elem 9
LDI r1 9
STR r1 rf
INC r1
; Code
LDI r1 0 ; i
LDI r2 0 ; j
LDM r3 r2 ; [j]
LDI rF 1
ADD r4 r2 rF
LDM r4 r4 ; [j+1]
CMP rF r4 r3
BIR a15
LDI rF 100 ; not in order
STR rF r3
LDI rF 101
STR rF r4
LDM r3 rF
LDI rF 100
LDM r4 rF
INC r2
LDI rF 2
LDI rE 9
CMP rF r2 rE ; j < 9
BIR a2
LDI rF 4
CMP rF r1 rE ; i < 10
BIR a1

