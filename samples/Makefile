PREFIX=riscv64-unknown-elf-
AS=$(PREFIX)as
ASFLAGS=-march=rv32i
CC=$(PREFIX)gcc
CFLAGS=-march=rv32i -mabi=ilp32 -I. -nostdlib -nostartfiles -ffreestanding
LDFLAGS=-T link.ld
AR=$(PREFIX)ar
OBJCOPY=$(PREFIX)objcopy

.PHONY: clean

all: test.bin

test.elf: crt0.o test.o stdlib.a
	$(CC) $(LDFLAGS) $(CFLAGS) -o $@ $^

stdlib.a: stdlib.o
	$(AR) rcs $@ $^

%.bin: %.elf
	$(OBJCOPY) -Obinary $^ $@

%.o: %.s
	$(AS) $(ASFLAGS) -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $^

clean:
	rm -f *.o *.bin *.a
