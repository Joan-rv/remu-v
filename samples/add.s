    .org 0x0
    addi t0, x0, 1
loop:
    addi t0, t0, 1
    add t0, t0, t0
    beq x0, x0, loop
