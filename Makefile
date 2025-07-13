SAMPLEDIR=samples
ARGS?=$(SAMPLEDIR)/test.bin

.PHONY: all samples clean rust run

all: samples rust

run: samples
	cargo run -- $(ARGS)

rust:
	cargo build

samples:
	make -C $(SAMPLEDIR)

clean:
	cargo clean
	make -C $(SAMPLEDIR) clean
