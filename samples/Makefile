PREFIX=riscv64-unknown-elf-
AS=$(PREFIX)as
ASFLAGS=-march=rv32i
OBJCOPY=$(PREFIX)objcopy

.PHONY: all clean

all: add.bin

%.bin: %.o
	$(OBJCOPY) -Obinary $^ $@

%.o: %.s
	$(AS) $(ASFLAGS) -o $@ $^

clean:
	rm -f *.o *.bin
