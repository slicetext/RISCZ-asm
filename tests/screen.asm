LDI r1 255
SPG r1
LDI r2 0 ; counter
LDI r3 1 ; counter increment
LDI r4 224 ; red
LDI r5 0 ; pixel
LDI r6 2 ; pixel increment
LDI rF 16
DIV rF r5 rF ; Find row number
AND rF rF r3
ADD r5 rF r5 ; for checkerboard
STR r5 r4 ; draw pixel
SUB r5 r5 rF ; remove checkerboard offset
ADD r2 r2 r3 ; increment counter
ADD r5 r5 r6 ; increment pixel
LDI r1 254
LDI rF 5 ; not equal
CMP rF r1 r5 ; check if not written all pixels
BIR a8
CMP r0 r0 r0 ; true
BIR a14 ; infinite loop
NOP
