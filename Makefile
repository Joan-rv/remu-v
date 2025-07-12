PREFIX=riscv64-unknown-elf-
AS=$(PREFIX)as
ASFLAGS=-march=rv32i
OBJCOPY=$(PREFIX)objcopy

SAMPLEDIR=samples
ARGS?=$(SAMPLEDIR)/general.bin

.PHONY: all samples clean rust run

all: samples rust

samples: $(SAMPLEDIR)/general.bin

run: samples
	cargo run -- $(ARGS)

rust:
	cargo build

%.bin: %.o
	$(OBJCOPY) -Obinary $^ $@

%.o: %.s
	$(AS) $(ASFLAGS) -o $@ $^

clean:
	rm -f $(SAMPLEDIR)/*.o $(SAMPLEDIR)/*.bin
