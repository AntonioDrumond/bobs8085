start:  jmp end
        mvi h,02
        mvi h,021
        mvi h,021h
        mvi h,021H
        mvi h,0x021
end:    jmp start
        jmp 10000
        hlt
