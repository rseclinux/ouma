.text

.globl rs_strfroml
.type rs_strfroml,@function
rs_strfroml:
        pushl   %ebp
        movl    %esp, %ebp
        subl    $24, %esp
        fldt    20(%ebp)
        fstpt   -24(%ebp)
        movq    -24(%ebp), %xmm0
        movl    -16(%ebp), %eax
        movq    %xmm0, 20(%ebp)
        movl    %eax, 28(%ebp)
        leave
        jmp     __oumainternal_strfromenc
.size rs_strfroml, .-rs_strfroml

.globl rs_strtold
.type rs_strtold,@function
rs_strtold:
        pushl   %ebp
        movl    %esp, %ebp
        subl    $44, %esp
        leal    -20(%ebp), %eax
        pushl   12(%ebp)
        pushl   8(%ebp)
        pushl   %eax
        call    __oumainternal_strtofloatenc
        movq    -20(%ebp), %xmm0
        movl    -12(%ebp), %eax
        movq    %xmm0, -40(%ebp)
        movl    %eax, -32(%ebp)
        fldt    -40(%ebp)
        leave
        ret
.size rs_strtold, .-rs_strtold

.globl rs_strtold_l
.type rs_strtold_l,@function
rs_strtold_l:
        pushl   %ebp
        movl    %esp, %ebp
        subl    $40, %esp
        leal    -20(%ebp), %eax
        pushl   16(%ebp)
        pushl   12(%ebp)
        pushl   8(%ebp)
        pushl   %eax
        call    __oumainternal_strtofloatenc_l
        movq    -20(%ebp), %xmm0
        movl    -12(%ebp), %eax
        movq    %xmm0, -40(%ebp)
        movl    %eax, -32(%ebp)
        fldt    -40(%ebp)
        leave
        ret
.size rs_strtold_l, .-rs_strtold_l

.globl rs_wcstold
.type rs_wcstold,@function
rs_wcstold:
        pushl   %ebp
        movl    %esp, %ebp
        subl    $44, %esp
        leal    -20(%ebp), %eax
        pushl   12(%ebp)
        pushl   8(%ebp)
        pushl   %eax
        call    __oumainternal_wcstofloatenc
        movq    -20(%ebp), %xmm0
        movl    -12(%ebp), %eax
        movq    %xmm0, -40(%ebp)
        movl    %eax, -32(%ebp)
        fldt    -40(%ebp)
        leave
        ret
.size rs_wcstold, .-rs_wcstold

.globl rs_wcstold_l
.type rs_wcstold_l,@function
rs_wcstold_l:
        pushl   %ebp
        movl    %esp, %ebp
        subl    $40, %esp
        leal    -20(%ebp), %eax
        pushl   16(%ebp)
        pushl   12(%ebp)
        pushl   8(%ebp)
        pushl   %eax
        call    __oumainternal_wcstofloatenc_l
        movq    -20(%ebp), %xmm0
        movl    -12(%ebp), %eax
        movq    %xmm0, -40(%ebp)
        movl    %eax, -32(%ebp)
        fldt    -40(%ebp)
        leave
        ret
.size rs_wcstold_l, .-rs_wcstold_l
