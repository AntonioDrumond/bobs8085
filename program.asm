//Store from memory location C020 five consecutive nos to be sorted in ascending orde

START: 
                             MVI D,05H	//Counter
                             
W: 
                             LXI H,C020H
                             
                             
                             MVI C,05H	//Counter
                             
X: 
                             MOV A,M
                             INX H
                             MOV B,M
                             CMP B
                             JM Y
                             
                             
                             MOV M,A
                             DCX H
                             MOV M,B
                             INX H
Y: 
                             DCR C
                             JNZ X
                             
                             
                             DCR D
                             JNZ W
                             
                             
                             HLT
