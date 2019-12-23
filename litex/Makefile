ELF=target/riscv32imac-unknown-none-elf/debug/litex 

run: litex.bin
	(cd ..; ./rust.py --trace --trace-start=1076000 --trace-end=1100000)

litex.bin: $(ELF)
	riscv64-unknown-elf-objcopy -O binary $(ELF) $@

$(ELF): src/main.rs
	cargo build

dis:
	riscv64-unknown-elf-objdump -D $(ELF) | less

view:
	gtkwave ../build/sim/gateware/start.gtkw # dut.vcd
