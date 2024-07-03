fn main() {
    use std::{
        env, fs,
        path::{Path, PathBuf},
    };

    let base_address = 0x8020_0000usize;
    let boot_file_content = fs::read_to_string("src/boot.rs").unwrap();
    let smp_reg = regex::Regex::new("pub const SMP: usize = [0-9]+;").unwrap();
    let smp_config = format!("pub const SMP: usize = {};", env::var("SMP").unwrap());
    let boot_file_content = smp_reg.replace(&boot_file_content, smp_config).to_string();
    let boot_file_path = Path::new("src/boot.rs");
    fs::write(boot_file_path, boot_file_content).unwrap();

    let ld = &PathBuf::from(env::var("OUT_DIR").unwrap()).join("linker.ld");
    fs::write(
        ld,
        format!(
            "\
OUTPUT_ARCH(riscv)
BASE_ADDRESS = {base_address};
ENTRY(_start)
SECTIONS
{{
    . = BASE_ADDRESS;
    _skernel = .;

    .text : ALIGN(4K) {{
        _stext = .;
        *(.text.boot)
        . = ALIGN(4K);
        *(.text.signal_trampoline)
        . = ALIGN(4K);
        *(.text .text.*)
        . = ALIGN(4K);
        _etext = .;
    }}

    .rodata : ALIGN(4K) {{
        _srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        *(.sdata2 .sdata2.*)
        . = ALIGN(4K);
        _erodata = .;
    }}

    .data : ALIGN(4K) {{
        _sdata = .;
        *(.data.boot_page_table)
        . = ALIGN(4K);
        _img_start = .;
        . = ALIGN(4K);
        _img_end = .;
        . = ALIGN(4K);
        *(.data .data.*)
        *(.sdata .sdata.*)
        *(.got .got.*)
	_initcall = .;
	KEEP(*(.initcall))
	_initcall_end =.;
    }}

    .tdata : ALIGN(0x10) {{
        _stdata = .;
        *(.tdata .tdata.*)
        _etdata = .;
    }}

    .tbss : ALIGN(0x10) {{
        _stbss = .;
        *(.tbss .tbss.*)
        *(.tcommon)
        _etbss = .;
    }}

    . = ALIGN(4K);
    _percpu_start = .;
    .percpu 0x0 : AT(_percpu_start) {{
        _percpu_load_start = .;
        *(.percpu .percpu.*)
        _percpu_load_end = .;
        . = ALIGN(64);
        _percpu_size_aligned = .;

        . = _percpu_load_start + _percpu_size_aligned * 4;
    }}
    . = _percpu_start + SIZEOF(.percpu);
    _percpu_end = .;

    . = ALIGN(4K);
    _edata = .;

    .bss : ALIGN(4K) {{
        boot_stack = .;
        *(.bss.stack)
        . = ALIGN(4K);
        boot_stack_top = .;

        _sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        *(COMMON)
        . = ALIGN(4K);
        _ebss = .;
    }}

    _ekernel = .;

	/DISCARD/ : {{
        *(.comment) *(.gnu*) *(.note*) *(.eh_frame*)
    }}
}}
"
        ),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/*.rs");
    println!("cargo:rustc-link-arg=-T{}", ld.display());
    println!("cargo:rustc-link-arg=-no-pie");
}
