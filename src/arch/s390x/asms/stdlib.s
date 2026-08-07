.text

.globl rs_strfroml
rs_strfroml:
        stmg    %r14,%r15,112(%r15)
        aghi    %r15,-176
        lg      %r0,0(%r5)
        lg      %r1,8(%r5)
        stg     %r1,168(%r15)
        stg     %r0,160(%r15)
        la      %r5,160(%r15)
        brasl   %r14,__oumainternal_strfromenc@PLT
        lg      %r4,288(%r15)
        lmg     %r14,%r15,288(%r15)
        br      %r4
.size rs_strfroml, .-rs_strfroml

.globl rs_strtold
rs_strtold:
        stmg    %r11,%r15,88(%r15)
        aghi    %r15,-176
        lgr     %r11,%r2
        la      %r2,160(%r15)
        brasl   %r14,__oumainternal_strtofloatenc@PLT
        ld      %f0,160(%r15)
        ld      %f2,168(%r15)
        lg      %r4,288(%r15)
        lgr     %r2,%r11
        std     %f0,0(%r11)
        std     %f2,8(%r11)
        lmg     %r11,%r15,264(%r15)
        br      %r4
.size rs_strtold, .-rs_strtold

.globl rs_strtold_l
rs_strtold_l:
        stmg    %r11,%r15,88(%r15)
        aghi    %r15,-176
        lgr     %r11,%r2
        la      %r2,160(%r15)
        brasl   %r14,__oumainternal_strtofloatenc_l@PLT
        ld      %f0,160(%r15)
        ld      %f2,168(%r15)
        lg      %r4,288(%r15)
        lgr     %r2,%r11
        std     %f0,0(%r11)
        std     %f2,8(%r11)
        lmg     %r11,%r15,264(%r15)
        br      %r4
.size rs_strtold_l, .-rs_strtold_l

.globl rs_wcstold
rs_wcstold:
        stmg    %r11,%r15,88(%r15)
        aghi    %r15,-176
        lgr     %r11,%r2
        la      %r2,160(%r15)
        brasl   %r14,__oumainternal_wcstofloatenc@PLT
        ld      %f0,160(%r15)
        ld      %f2,168(%r15)
        lg      %r4,288(%r15)
        lgr     %r2,%r11
        std     %f0,0(%r11)
        std     %f2,8(%r11)
        lmg     %r11,%r15,264(%r15)
        br      %r4
.size rs_wcstold, .-rs_wcstold

.globl rs_wcstold_l
rs_wcstold_l:
        stmg    %r11,%r15,88(%r15)
        aghi    %r15,-176
        lgr     %r11,%r2
        la      %r2,160(%r15)
        brasl   %r14,__oumainternal_wcstofloatenc_l@PLT
        ld      %f0,160(%r15)
        ld      %f2,168(%r15)
        lg      %r4,288(%r15)
        lgr     %r2,%r11
        std     %f0,0(%r11)
        std     %f2,8(%r11)
        lmg     %r11,%r15,264(%r15)
        br      %r4
.size rs_wcstold_l, .-rs_wcstold_l
