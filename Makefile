LOG ?= info
SMP ?= 1
verbose ?=
TARGET_DIR = target/riscv64gc-unknown-none-elf
MODE := release
QEMU := qemu-system-riscv64
QEMU_NET := -device virtio-net-device,netdev=net0 -netdev user,id=net0,hostfwd=tcp::5555-:5555,hostfwd=udp::5555-:5555 -object filter-dump,id=net0,netdev=net0,file=dump.cap
QEMU_LOG := -D qemu.log -d in_asm,int

KERNEL_DIR := kernel
ELF := $(TARGET_DIR)/$(MODE)/kernel
ASM := $(TARGET_DIR)/$(MODE)/kernel.asm
BIN := $(TARGET_DIR)/$(MODE)/kernel.bin

OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

build:
	cd $(KERNEL_DIR) && LOG=$(LOG) SMP=$(SMP) cargo build --$(MODE) 
	$(OBJCOPY) $(ELF) --strip-all -O binary $(BIN)

run: build
	$(QEMU) -smp $(SMP) -m 128M -machine virt --nographic -bios default --kernel $(BIN) $(QEMU_NET)

disasm: build
	$(OBJDUMP) -S -t $(ELF) > $(ASM)

clean:
	rm -rf target
