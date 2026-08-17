.text

.globl rs_strfroml
.type rs_strfroml,@function
rs_strfroml:
        push    {r0, r1, r2, r4}
        ldrd    r2, r3, [sp, #16]
        strd    r2, r3, [sp]
        str     r4, [sp, #16]
        add     sp, sp, #12
        pop     {r4}
        b       __oumainternal_strfromenc
.size rs_strfroml, .-rs_strfroml

.globl rs_strtold
.type rs_strtold,@function
rs_strtold:
        push    {r0, r1, r2, lr}
        mov     r2, r1
        mov     r1, r0
        mov     r0, sp
        bl      __oumainternal_strtofloatenc
        ldrd    r0, [sp]
        add     sp, sp, #12
        pop     {pc}
.size rs_strtold, .-rs_strtold

.globl rs_strtold_l
.type rs_strtold_l,@function
rs_strtold_l:
        push    {r0, r1, r2, lr}
        mov     r3, r2
        mov     r2, r1
        mov     r1, r0
        mov     r0, sp
        bl      __oumainternal_strtofloatenc_l
        ldrd    r0, [sp]
        add     sp, sp, #12
        pop     {pc}
.size rs_strtold_l, .-rs_strtold_l

.globl rs_wcstold
.type rs_wcstold,@function
rs_wcstold:
        push    {r0, r1, r2, lr}
        mov     r2, r1
        mov     r1, r0
        mov     r0, sp
        bl      __oumainternal_wcstofloatenc
        ldrd    r0, [sp]
        add     sp, sp, #12
        pop     {pc}
.size rs_wcstold, .-rs_wcstold

.globl rs_wcstold_l
.type rs_wcstold_l,@function
rs_wcstold_l:
        push    {r0, r1, r2, lr}
        mov     r3, r2
        mov     r2, r1
        mov     r1, r0
        mov     r0, sp
        bl      __oumainternal_wcstofloatenc_l
        ldrd    r0, [sp]
        add     sp, sp, #12
        pop     {pc}
.size rs_wcstold_l, .-rs_wcstold_l
