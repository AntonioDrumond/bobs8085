// From Jubin Mitra's 8085 Simulator

//1's COMPLEMENT OF A 16-BIT NUMBER
//The 16bit number is stored in C050,C051
//The answer is stored in C052,C053

LXI H,C050H
MOV A,M
CMA
STA C052H
INX H
MOV A,M
CMA 
STA C053H
HLT

//EXAMPLE-> C050=85,C051=54
//Answer-> C052=7A,C053=AB