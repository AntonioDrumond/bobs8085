
//Fibonacci Series Generation
//To run the Program simply load at memory location C050=01,C051=01
START: 
                             MVI C,09H		//Counter
                             
                             LXI H,C050H                            	//Memory Pointer
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
                             
                             
                             RST 1
