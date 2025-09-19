START:
    ; Initialize registers
    LXI H,DATA         ; HL -> DATA
    LXI D,1234h       ; D,E = 12,34h
    LXI B,5678h       ; B,C = 56,78h
    LXI SP,STACK       ; Setup stack pointer
    MVI A,55h         ; A = 55h
    MVI M,AAh         ; (HL) = AAh

    ; Move and load/store tests
    MOV B,A
    MOV C,B
    MOV D,C
    MOV E,D
    MOV H,E
    MOV L,H
    MOV A,L
    STAX B              ; Store A at (BC)
    LDAX D              ; Load A from (DE)
    SHLD DATA2          ; Store HL pair
    LHLD DATA2          ; Reload HL pair
    XCHG                ; Swap HL and DE

    ; Stack operations
    PUSH B
    PUSH D
    PUSH H
    POP H
    POP D
    POP B
    XTHL                    ; Exchange top of stack with HL
    SPHL                    ; Load SP with HL

    ; Increment/Decrement
    INX B
    INX D
    INX H
    INX SP
    DCX B
    DCX D
    DCX H
    DCX SP

    ; ALU operations (register mode)
    MVI A,01h
    MVI B,02h
    ADD B
    ADC B
    SUB B
    SBB B
    ANA B
    XRA B
    ORA B
    CMP B

    ; ALU immediate mode
    ADI 10h
    ACI 01h
    SUI 02h
    SBI 01h
    ANI F0h
    XRI 0Fh
    ORI 01h
    CPI 10h

    ; Rotate and special instructions
    RLC
    RRC
    RAL
    RAR
    CMA
    STC
    CMC
    DAA

    ; I/O operations (ports are arbitrary)
    OUT 01h
    IN 01h

    ; Conditional jumps
    MVI A,00h
    CPI 00h
    JZ  ZERO_LABEL
    JMP NONZERO_LABEL

ZERO_LABEL:
    MVI A,FFh
    JNZ NONZERO_LABEL

NONZERO_LABEL:
    JC CARRY_LABEL
CARRY_LABEL:
    JNC NOCARRY_LABEL
NOCARRY_LABEL:
    JP POS_LABEL
POS_LABEL:
    JM NEG_LABEL
NEG_LABEL:
    JPE EVEN_LABEL
EVEN_LABEL:
    JPO ODD_LABEL
ODD_LABEL:
    PCHL                    ; Jump via HL (HL still points to DATA)

    ; Calls and returns
    CALL SUBR
    CC SUBR
    CNC SUBR
    CZ SUBR
    CNZ SUBR
    CP SUBR
    CM SUBR
    CPE SUBR
    CPO SUBR

    RET
    RC
    RNC
    RZ
    RNZ
    RP
    RM
    RPE
    RPO

    RST 0
    RST 1
    RST 2
    RST 3
    RST 4
    RST 5
    RST 6
    RST 7

    ; Interrupt control
    EI
    DI

    NOP
    RIM
    SIM

    HLT                     ; End program

SUBR:
    INR A
    DCR A
    DAD B
    RET

; Data section
DATA:   NOP ; DB  0x00  
DATA2:  NOP ; DS  2

; Reserve stack memory
STACK:  NOP ; DS  32