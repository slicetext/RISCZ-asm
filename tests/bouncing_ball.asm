LDI r1 2 ; coord
LDI r2 1 ; x direction: 1 is left, 2 is right
LDI r3 1 ; y direction: 1 is up, 2 is down
LDI rF 255
SPG rF ; setpage screen

LDI rD 1 ; entire row
LDI rF 0 ; equals
LDI rE 1
CMP rF rE r2 ; if x direction is left
BIR a13 ; this means x is going right
ADD r1 r1 rD
LDI rE 2
CMP rF rE r2 ; if x direction is right
BIR a17 ; this means x is going left
SUB r1 r1 rD

LDI rD 16 ; entire row
LDI rE 1
CMP rF rE r3 ; if y direction is up
BIR a23 ; this means y is going down
ADD r1 r1 rD
LDI rE 2
CMP rF rE r3 ; if y direction is down
BIR a27 ; this means x is going up
SUB r1 r1 rE

LDI rF 255
STR r1 rF
CMP r0 r0 r0
BIR a7
