.text

.globl rs_strfroml
.type rs_strfroml,@function
rs_strfroml:
        addi    sp,sp,-16
        sd      a3,0(sp)
        sd      a4,8(sp)
        addi    sp,sp,16
        tail    __oumainternal_strfromenc@plt
.size rs_strfroml, .-rs_strfroml

.globl rs_strtold
.type rs_strtold,@function
rs_strtold:
        addi    sp,sp,-32
        sd      ra,24(sp)
        call    __oumainternal_strtofloatenc@plt
        ld      ra,24(sp)
        addi    sp,sp,32
        jr      ra
.size rs_strtold, .-rs_strtold

.globl rs_strtold_l
.type rs_strtold_l,@function
rs_strtold_l:
        addi    sp,sp,-32
        sd      ra,24(sp)
        call    __oumainternal_strtofloatenc_l@plt
        ld      ra,24(sp)
        addi    sp,sp,32
        jr      ra
.size rs_strtold_l, .-rs_strtold_l

.globl rs_wcstold
.type rs_wcstold,@function
rs_wcstold:
        addi    sp,sp,-32
        sd      ra,24(sp)
        call    __oumainternal_wcstofloatenc@plt
        ld      ra,24(sp)
        addi    sp,sp,32
        jr      ra
.size rs_wcstold, .-rs_wcstold

.globl rs_wcstold_l
.type rs_wcstold_l,@function
rs_wcstold_l:
        addi    sp,sp,-32
        sd      ra,24(sp)
        call    __oumainternal_wcstofloatenc_l@plt
        ld      ra,24(sp)
        addi    sp,sp,32
        jr      ra
.size rs_wcstold_l, .-rs_wcstold_l
