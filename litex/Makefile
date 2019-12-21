ELF=target/riscv32imac-unknown-none-elf/debug/litex 

litex.bin: $(ELF)
	riscv64-unknown-elf-objcopy -O binary $(ELF) $@

$(ELF): src/main.rs
	cargo build

dis:
	riscv64-unknown-elf-objdump -D $(ELF) | less

