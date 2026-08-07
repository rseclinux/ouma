.text

.globl rs_strfroml
rs_strfroml:
        addi    sp,sp,-48
        sw      s2,32(sp)
        mv      s2,a2
        lw      a2,4(a3)
        sw      s1,36(sp)
        mv      s1,a1
        lw      a1,0(a3)
        lw      a4,8(a3)
        lw      a5,12(a3)
        sw      a2,20(sp)
        li      a2,16
        sw      s0,40(sp)
        sw      a1,16(sp)
        mv      s0,a0
        add     a1,sp,a2
        mv      a0,sp
        sw      ra,44(sp)
        sw      a4,24(sp)
        sw      a5,28(sp)
        call    memcpy@plt
        mv      a3,sp
        mv      a2,s2
        mv      a1,s1
        mv      a0,s0
        call    __oumainternal_strfromenc@plt
        lw      ra,44(sp)
        lw      s0,40(sp)
        lw      s1,36(sp)
        lw      s2,32(sp)
        addi    sp,sp,48
        jr      ra
.size rs_strfroml, .-rs_strfroml

.globl rs_strtold
rs_strtold:
        addi    sp,sp,-32
        sw      s0,24(sp)
        mv      s0,a0
        mv      a0,sp
        sw      ra,28(sp)
        call    __oumainternal_strtofloatenc@plt
        lw      a5,0(sp)
        lw      ra,28(sp)
        mv      a0,s0
        sw      a5,0(s0)
        lw      a5,4(sp)
        sw      a5,4(s0)
        lw      a5,8(sp)
        sw      a5,8(s0)
        lw      a5,12(sp)
        sw      a5,12(s0)
        lw      s0,24(sp)
        addi    sp,sp,32
        jr      ra
.size rs_strtold, .-rs_strtold

.globl rs_strtold_l
rs_strtold_l:
        addi    sp,sp,-32
        sw      s0,24(sp)
        mv      s0,a0
        mv      a0,sp
        sw      ra,28(sp)
        call    __oumainternal_strtofloatenc_l@plt
        lw      a5,0(sp)
        lw      ra,28(sp)
        mv      a0,s0
        sw      a5,0(s0)
        lw      a5,4(sp)
        sw      a5,4(s0)
        lw      a5,8(sp)
        sw      a5,8(s0)
        lw      a5,12(sp)
        sw      a5,12(s0)
        lw      s0,24(sp)
        addi    sp,sp,32
        jr      ra
.size rs_strtold_l, .-rs_strtold_l

.globl rs_wcstold
rs_wcstold:
        addi    sp,sp,-32
        sw      s0,24(sp)
        mv      s0,a0
        mv      a0,sp
        sw      ra,28(sp)
        call    __oumainternal_wcstofloatenc@plt
        lw      a5,0(sp)
        lw      ra,28(sp)
        mv      a0,s0
        sw      a5,0(s0)
        lw      a5,4(sp)
        sw      a5,4(s0)
        lw      a5,8(sp)
        sw      a5,8(s0)
        lw      a5,12(sp)
        sw      a5,12(s0)
        lw      s0,24(sp)
        addi    sp,sp,32
        jr      ra
.size rs_wcstold, .-rs_wcstold

.globl rs_wcstold_l
rs_wcstold_l:
        addi    sp,sp,-32
        sw      s0,24(sp)
        mv      s0,a0
        mv      a0,sp
        sw      ra,28(sp)
        call    __oumainternal_wcstofloatenc_l@plt
        lw      a5,0(sp)
        lw      ra,28(sp)
        mv      a0,s0
        sw      a5,0(s0)
        lw      a5,4(sp)
        sw      a5,4(s0)
        lw      a5,8(sp)
        sw      a5,8(s0)
        lw      a5,12(sp)
        sw      a5,12(s0)
        lw      s0,24(sp)
        addi    sp,sp,32
        jr      ra
.size rs_wcstold_l, .-rs_wcstold_l
