    .org 0x0
    addi t0, x0, 1
loop:
    addi t0, t0, 1
    add t0, t0, t0
    blt x0, t0, loop
    addi t0, x0, 100
    lw t1, -1(t0)
    lh t2, -1(t0)
    lb t3, -1(t0)
    lhu t4, -1(t0)
    lbu t5, -1(t0)
