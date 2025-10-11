LDI r1 10 ; number to get fibonacci of
LDI r2 0 ; result

LDI rF 1 ; Greater than
LDI rE 1 ; Number to compare to
CMP rF r1 rE ; If r1 is greater than 1
BIR 9 ; skip over base case
LDI r2 1
RET
LDI rF 1 ; Load 1 into rF for decrement
SUB r1 r1 rF ; decrement r1
LDI rE 1
CMP rF rF rE ; known true comparison
BIR 3 ; jump to start of function
STR rF r2 ; store result in mem location 1
LDM r3 rF ; store result in r3, finishing copy
SUB r1 r1 rF
CMP rF rF rE
BIR 3
ADD r2 r2 r3
ADD r1 r1 rF ; Increment r1 for parent calls

NOP ; Program done
