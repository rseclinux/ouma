.globl rs_strfroml
rs_strfroml:
        fldt    8(%rsp)
        fstpt   -24(%rsp)
        movq    -24(%rsp), %rcx
        movq    -16(%rsp), %r8
        jmp     __oumainternal_strfromenc
.size rs_strfroml, .-rs_strfroml

.globl rs_strtold
rs_strtold:
        subq    $24, %rsp
        call    __oumainternal_strtofloatenc
        movq    %rax, (%rsp)
        movq    %rdx, 8(%rsp)
        fldt    (%rsp)
        addq    $24, %rsp
        ret
.size rs_strtold, .-rs_strtold

.globl rs_strtold_l
rs_strtold_l:
        subq    $24, %rsp
        call    __oumainternal_strtofloatenc_l
        movq    %rax, (%rsp)
        movq    %rdx, 8(%rsp)
        fldt    (%rsp)
        addq    $24, %rsp
        ret
.size rs_strtold_l, .-rs_strtold_l

.globl rs_wcstold
rs_wcstold:
        subq    $24, %rsp
        call    __oumainternal_wcstofloatenc
        movq    %rax, (%rsp)
        movq    %rdx, 8(%rsp)
        fldt    (%rsp)
        addq    $24, %rsp
        ret
.size rs_wcstold, .-rs_wcstold

.globl rs_wcstold_l
rs_wcstold_l:
        subq    $24, %rsp
        call    __oumainternal_wcstofloatenc_l
        movq    %rax, (%rsp)
        movq    %rdx, 8(%rsp)
        fldt    (%rsp)
        addq    $24, %rsp
        ret
.size rs_wcstold_l, .-rs_wcstold_l
