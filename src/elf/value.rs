// ELFHeader.e_type
pub const ET_NONE: u16 = 0;
pub const ET_REL: u16 = 1;
pub const ET_EXEC: u16 = 2;
pub const ET_DYN: u16 = 3;
pub const ET_CORE: u16 = 4;
pub const ET_LOPROC: u16 = 0xff00;
pub const ET_HIPROC: u16 = 0xffff;

// ELFHeader.e_machine
pub const EM_NONE: u16 = 0;
pub const EM_M32: u16 = 1;
pub const EM_SPARC: u16 = 2;
pub const EM_386: u16 = 3;
pub const EM_68K: u16 = 4;
pub const EM_88K: u16 = 5;
pub const EM_860: u16 = 7;
pub const EM_MIPS: u16 = 8;
pub const EM_MIPS_RS4_BE: u16 = 10;

// ELFHeader.e_version
pub const EV_NONE: u32 = 0;
pub const EV_CURRENT: u32 = 1;

// ELFHeader.e_ident index
pub const EI_MAG0: usize = 0;
pub const EI_MAG1: usize = 1;
pub const EI_MAG2: usize = 2;
pub const EI_MAG3: usize = 3;
pub const EI_CLASS: usize = 4;
pub const EI_DATA: usize = 5;
pub const EI_VERSION: usize = 6;
pub const EI_PAD: usize = 7;
pub const EI_NIDENT: usize = 16;

// EI_MAG
pub const ELFMAG0: u8 = 0x7F;
pub const ELFMAG1: u8 = b'E';
pub const ELFMAG2: u8 = b'L';
pub const ELFMAG3: u8 = b'F';

// EI_CLASS
pub const ELFCLASSNONE: u8 = 0;
pub const ELFCLASS32: u8 = 1;
pub const ELFCLASS64: u8 = 2;

// EI_DATA
pub const ELFDATANONE: u8 = 0;
pub const ELFDATA2LSB: u8 = 1;
pub const ELFDATA2MSB: u8 = 2;

// special section index
pub const SHN_UNDEF: usize = 0;
pub const SHN_LORESERVE: usize = 0xFF00;
pub const SHN_LOPROC: usize = 0xFF00;
pub const SHN_HIPROC: usize = 0xFF1F;
pub const SHN_ABS: usize = 0xFFF1;
pub const SHN_COMMON: usize = 0xFFF2;
pub const SHN_HIRESERVE: usize = 0xFFFF;

// SectionHeader.sh_type
pub const SHT_NULL: u32 = 0;
pub const SHT_PROGBITS: u32 = 1;
pub const SHT_SYMTAB: u32 = 2;
pub const SHT_STRTAB: u32 = 3;
pub const SHT_RELA: u32 = 4;
pub const SHT_HASH: u32 = 5;
pub const SHT_DYNAMIC: u32 = 6;
pub const SHT_NOTE: u32 = 7;
pub const SHT_NOBITS: u32 = 8;
pub const SHT_REL: u32 = 9;
pub const SHT_SHLIB: u32 = 10;
pub const SHT_DYNSYM: u32 = 11;
pub const SHT_LOPROC: u32 = 0x70000000;
pub const SHT_HIPROC: u32 = 0x7fffffff;
pub const SHT_LOUSER: u32 = 0x80000000;
pub const SHT_HIUSER: u32 = 0xffffffff;

// SectionHeader.sh_flags
pub const SHF_WRITE: u32 = 0x1;
pub const SHF_ALLOC: u32 = 0x2;
pub const SHF_EXECINSTR: u32 = 0x4;
pub const SHF_MASKPROC: u32 = 0xF0000000;

// symbol table index
pub const STN_UNDEF: usize = 0;

// symbol info macro
pub fn symbol_bind(i: u8) -> u8 {
    i >> 4
}
pub fn symbol_type(i: u8) -> u8 {
    i & 0xF
}
pub fn symbol_info(b: u8, t: u8) -> u8 {
    (b << 4) | (t & 0xF)
}

// symbol bind
pub const STB_LOCAL: u8 = 0;
pub const STB_GLOBAL: u8 = 1;
pub const STB_WEAK: u8 = 2;
pub const STB_LOPROC: u8 = 13;
pub const STB_HIPROC: u8 = 15;

// symbol type
pub const STT_NOTYPE: u8 = 0;
pub const STT_OBJECT: u8 = 1;
pub const STT_FUNC: u8 = 2;
pub const STT_SECTION: u8 = 3;
pub const STT_FILE: u8 = 4;
pub const STT_LOPROC: u8 = 13;
pub const STT_HIPROC: u8 = 15;

// rel & rela macro
pub fn relocation_symbol(i: u32) -> u32 {
    i >> 8
}
pub fn relocation_type(i: u32) -> u32 {
    i & 0xF
}
pub fn relocation_info(s: u32, t: u32) -> u32 {
    (s << 8) | (t & 0xF)
}
