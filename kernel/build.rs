
fn main() {
    use std::{env, fs, path::{Path, PathBuf}};

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
ENTRY(__entry)
SECTIONS {{
    . = {base_address};
    skernel = .;
    stext = .;
    .text : {{
        *(.text.entry)
        *(.text .text.*)
    }}

    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {{
        *(.rodata .rodata.*)
    }}

    . = ALIGN(4K);
    erodata = .;
    s_data = .;
    .data : {{
        *(.data .data.*)
    }}
    e_data = .;

    . = ALIGN(4K);
    e_data = .;
    .bss : {{
        *(.bss.stack)
        sbss = .;
        *(.sbss .bss .bss.*)
        ebss = .;
    }}

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {{
        *(.eh_frame)
    }}
}}"
        ),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/*.rs");
    println!("cargo:rustc-link-arg=-T{}", ld.display());
}
