    .org 0x0
    addi t0, x0, 1
loop:
    addi t0, t0, 1
    add t0, t0, t0
    blt x0, t0, loop
