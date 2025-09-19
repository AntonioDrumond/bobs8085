					START: 
							 MVI A,01h
							 STA c050h
							 STA c051h
							 MVI A,0h
                             MVI C,09h		//Counter
                             
                             LXI H,C050h                            	//Memory Pointer
					X: 
                             MOV A,M
                             INX H
                             MOV B,M
                             INX H
                             ADD B
                             DAA
                             MOV M,A
                             DCX H
                             DCR C
                             JNZ X
                             
                             HLT

