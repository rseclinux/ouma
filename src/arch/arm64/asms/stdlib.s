.text

.globl rs_strfroml
rs_strfroml:
        fmov    x4, d0
        fmov    x5, v0.d[1]
        mov     x3, x4
        mov     x4, x5
        b       __oumainternal_strfromenc
.size rs_strfroml, .-rs_strfroml

.globl rs_strtold
rs_strtold:
        stp     x29, x30, [sp, -32]!
        mov     x29, sp
        bl      __oumainternal_strtofloatenc
        stp     x0, x1, [sp, 16]
        ldr     q0, [sp, 16]
        ldp     x29, x30, [sp], 32
        ret
.size rs_strtold, .-rs_strtold

.globl rs_strtold_l
rs_strtold_l:
        stp     x29, x30, [sp, -32]!
        mov     x29, sp
        bl      __oumainternal_strtofloatenc_l
        stp     x0, x1, [sp, 16]
        ldr     q0, [sp, 16]
        ldp     x29, x30, [sp], 32
        ret
.size rs_strtold_l, .-rs_strtold_l

.globl rs_wcstold
rs_wcstold:
        stp     x29, x30, [sp, -32]!
        mov     x29, sp
        bl      __oumainternal_wcstofloatenc
        stp     x0, x1, [sp, 16]
        ldr     q0, [sp, 16]
        ldp     x29, x30, [sp], 32
        ret
.size rs_wcstold, .-rs_wcstold

.globl rs_wcstold_l
rs_wcstold_l:
        stp     x29, x30, [sp, -32]!
        mov     x29, sp
        bl      __oumainternal_wcstofloatenc_l
        stp     x0, x1, [sp, 16]
        ldr     q0, [sp, 16]
        ldp     x29, x30, [sp], 32
        ret
.size rs_wcstold_l, .-rs_wcstold_l
