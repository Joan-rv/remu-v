    .org 0x0
    addi t0, x0, 1
loop:
    addi t0, t0, 1
    add t0, t0, t0
    blt x0, t0, loop
    lw t1, 0(x0)
    lh t2, 0(x0)
    lb t3, 0(x0)
    lhu t4, 0(x0)
    lbu t5, 0(x0)
