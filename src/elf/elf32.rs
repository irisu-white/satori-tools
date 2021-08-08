use bytes::Buf;
use std::io::{
    prelude::*,
    SeekFrom,
};
use crate::elf::value;


#[derive(Debug, Default)]
pub struct ELFHeader {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl ELFHeader {
    pub fn new() -> ELFHeader {
        Self::default()
    }

    pub fn load<T>(elf: &mut T) -> Result<ELFHeader, String>
        where T: Read
    {
        // sizeof(ELFHeader) == 0x34
        let mut data: [u8; 0x34] = [0; 0x34];
        match elf.read(&mut data) {
            Ok(n) => if n != 0x34 { return Err(String::from("invalid length")); },
            Err(e) => return Err(e.to_string()),
        };

        let mut cursor = &data[..];
        let mut e_ident: [u8; 16] = [0; 16];
        cursor.copy_to_slice(&mut e_ident);
        let e_type: u16 = cursor.get_u16_le();
        let e_machine: u16 = cursor.get_u16_le();
        let e_version: u32 = cursor.get_u32_le();
        let e_entry: u32 = cursor.get_u32_le();
        let e_phoff: u32 = cursor.get_u32_le();
        let e_shoff: u32 = cursor.get_u32_le();
        let e_flags: u32 = cursor.get_u32_le();
        let e_ehsize: u16 = cursor.get_u16_le();
        let e_phentsize: u16 = cursor.get_u16_le();
        let e_phnum: u16 = cursor.get_u16_le();
        let e_shentsize: u16 = cursor.get_u16_le();
        let e_shnum: u16 = cursor.get_u16_le();
        let e_shstrndx: u16 = cursor.get_u16_le();
        
        Ok(ELFHeader {
            e_ident, e_type, e_machine, e_version,
            e_entry, e_phoff, e_shoff, e_flags,
            e_ehsize, e_phentsize, e_phnum,
            e_shentsize, e_shnum, e_shstrndx,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct SectionHeader {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u32,
    pub sh_addr: u32,
    pub sh_offset: u32,
    pub sh_size: u32,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u32,
    pub sh_entsize: u32,
}

#[derive(Debug, Default)]
pub struct SectionTable {
    // ELFHeader.e_shoff
    pub offset: u32,
    // ELFHeader.e_shentsize
    pub size: u16,
    // ELFHeader.e_shnum
    pub num: u16,
    // ELFHeader.e_shstrndx
    pub name: u16,
    pub data: Vec<SectionHeader>,
}

impl SectionTable {
    pub fn new() -> SectionTable {
        Self::default()
    }

    pub fn load<T>(elf: &mut T, ehdr: &ELFHeader) -> Result<SectionTable, String>
        where T: Read + Seek
    {
        // check e_shentsize, min size is 0x28
        if ehdr.e_shentsize < 0x28 {
            return Err(String::from("invalid e_shentsize"));
        }
        let num = ehdr.e_shnum as u64;
        let size = ehdr.e_shentsize as u64;
        let offset = ehdr.e_shoff as u64;

        let mut data: [u8; 0x28] = [0; 0x28];
        let mut sections: Vec<SectionHeader> = Vec::with_capacity(num as usize);

        for i in 0..num {
            // read data
            let pos: u64 = offset + i * size;
            if let Err(e) = elf.seek(std::io::SeekFrom::Start(pos)) {
                return Err(e.to_string());
            }
            match elf.read(&mut data) {
                Ok(n) => if n != 0x28 { return Err(String::from("invalid length")) },
                Err(e) => return Err(e.to_string()),
            };
            // parse section header
            let mut cursor = &data[..];
            let sh_name: u32 = cursor.get_u32_le();
            let sh_type: u32 = cursor.get_u32_le();
            let sh_flags: u32 = cursor.get_u32_le();
            let sh_addr: u32 = cursor.get_u32_le();
            let sh_offset: u32 = cursor.get_u32_le();
            let sh_size: u32 = cursor.get_u32_le();
            let sh_link: u32 = cursor.get_u32_le();
            let sh_info: u32 = cursor.get_u32_le();
            let sh_addralign: u32 = cursor.get_u32_le();
            let sh_entsize: u32 = cursor.get_u32_le();
            sections.push(SectionHeader {
                sh_name, sh_type, sh_flags, sh_addr,
                sh_offset, sh_size, sh_link, sh_info,
                sh_addralign, sh_entsize,
            });
        }

        Ok(SectionTable {
            offset: ehdr.e_shoff,
            size: ehdr.e_shentsize,
            num: ehdr.e_shnum,
            name: ehdr.e_shstrndx,
            data: sections,
        })
    }
}

#[derive(Debug, Default)]
pub struct StringTable {
    pub section: SectionHeader,
    pub data: Vec<u8>,
}

impl StringTable {
    pub fn new() -> StringTable {
        Self::default()
    }

    pub fn load<T>(elf: &mut T, section: &SectionHeader) -> Result<StringTable, String>
        where T: Read + Seek
    {
        // check section type
        if section.sh_type != value::SHT_STRTAB {
            return Err(String::from("section type is not SHT_STRTAB"));
        }
        // read data
        let offset = section.sh_offset;
        if let Err(e) = elf.seek(SeekFrom::Start(offset as u64)) {
            return Err(e.to_string());
        }
        let size = section.sh_size;
        let mut data: Vec<u8> = Vec::new();
        data.resize(size as usize, 0);
        match elf.read(&mut data) {
            Ok(n) => if n != size as usize { return Err(String::from("invalid length")); },
            Err(e) => return Err(e.to_string()),
        };

        Ok(StringTable {
            section: section.clone(),
            data
        })
    }

    pub fn get(&self, pos: usize) -> String {
        if pos >= self.data.len() {
            panic!("index of StringTable out of range");
        }
        let mut s: Vec<u8> = Vec::new();
        for n in self.data.iter().skip(pos) {
            if *n != b'\0' {
                s.push(*n);
            }
            else {
                break;
            }
        }
        match String::from_utf8(s) {
            Ok(s) => return s,
            Err(_) => panic!("invalid StringTable"),
        };
    }
}

#[derive(Debug, Default)]
pub struct SymbolEntry {
    pub st_name: u32,
    pub st_value: u32,
    pub st_size: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    pub section: SectionHeader,
    pub data: Vec<SymbolEntry>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        Self::default()
    }

    pub fn load<T>(elf: &mut T, section: &SectionHeader) -> Result<SymbolTable, String>
        where T: Read + Seek
    {
        // check section type
        if section.sh_type != value::SHT_SYMTAB {
            return Err(String::from("invalid section type"));
        }
        // sizeof(SymbolEntry) = 0x10
        if section.sh_entsize < 0x10 {
            return Err(String::from("invalid entry size"));
        }
        if section.sh_size % section.sh_entsize != 0 {
            return Err(String::from("invalid section size"));
        }

        let offset = section.sh_offset as u64;
        let entry_size = section.sh_entsize as u64;
        let num = (section.sh_size as u64) / entry_size;
        let mut data: [u8; 0x10] = [0; 0x10];
        let mut symbols: Vec<SymbolEntry> = Vec::with_capacity(num as usize);

        for n in 0..num {
            let pos = offset + n * entry_size;
            if let Err(e) = elf.seek(SeekFrom::Start(pos)) {
                return Err(e.to_string());
            }
            match elf.read(&mut data) {
                Ok(n) => if n != 0x10 { return Err(String::from("invalid length")); },
                Err(e) => return Err(e.to_string()),
            };
            // parse data
            let mut cursor = &data[..];
            let st_name = cursor.get_u32_le();
            let st_value = cursor.get_u32_le();
            let st_size = cursor.get_u32_le();
            let st_info = cursor.get_u8();
            let st_other = cursor.get_u8();
            let st_shndx = cursor.get_u16_le();
            symbols.push(SymbolEntry {
                st_name, st_value, st_size,
                st_info, st_other, st_shndx,
            });
        }

        Ok(SymbolTable {
            section: section.clone(),
            data: symbols,
        })
    }
}

#[derive(Debug, Default)]
pub struct RelEntry {
    pub r_offset: u32,
    pub r_info: u32,
}

#[derive(Debug, Default)]
pub struct RelTable {
    pub section: SectionHeader,
    pub data: Vec<RelEntry>,
}

impl RelTable {
    pub fn new() -> RelTable {
        Self::default()
    }

    pub fn load<T>(elf: &mut T, section: &SectionHeader) -> Result<RelTable, String>
        where T: Read + Seek
    {
        // check section type
        if section.sh_type != value::SHT_REL {
            return Err(String::from("invalid section type"));
        }
        // sizeof(RelEntry) = 0x8
        if section.sh_entsize < 0x8 {
            return Err(String::from("invalid entry size"));
        }
        if section.sh_size % section.sh_entsize != 0 {
            return Err(String::from("invalid section size"));
        }

        let offset = section.sh_offset as u64;
        let entry_size = section.sh_entsize as u64;
        let num = (section.sh_size as u64) / entry_size;
        let mut data: [u8; 0x8] = [0; 0x8];
        let mut entries: Vec<RelEntry> = Vec::with_capacity(num as usize);

        for n in 0..num {
            let pos = offset + n * entry_size;
            if let Err(e) = elf.seek(SeekFrom::Start(pos)) {
                return Err(e.to_string());
            }
            match elf.read(&mut data) {
                Ok(n) => if n != 0x8 { return Err(String::from("invalid length")); },
                Err(e) => return Err(e.to_string()),
            };
            // parse data
            let mut cursor = &data[..];
            let r_offset = cursor.get_u32_le();
            let r_info = cursor.get_u32_le();
            entries.push(RelEntry { r_offset, r_info });
        }

        Ok(RelTable {
            section: section.clone(),
            data: entries,
        })
    }
}

#[derive(Debug, Default)]
pub struct RelaEntry {
    pub r_offset: u32,
    pub r_info: u32,
    pub r_addend: i32,
}

#[derive(Debug, Default)]
pub struct RelaTable {
    pub section: SectionHeader,
    pub data: Vec<RelaEntry>,
}

impl RelaTable {
    pub fn new() -> RelaTable {
        Self::default()
    }

    pub fn load<T>(elf: &mut T, section: &SectionHeader) -> Result<RelaTable, String>
        where T: Read + Seek
    {
        // check section type
        if section.sh_type != value::SHT_RELA {
            return Err(String::from("invalid section type"));
        }
        // sizeof(RelEntry) = 0xC
        if section.sh_entsize < 0xC {
            return Err(String::from("invalid entry size"));
        }
        if section.sh_size % section.sh_entsize != 0 {
            return Err(String::from("invalid section size"));
        }

        let offset = section.sh_offset as u64;
        let entry_size = section.sh_entsize as u64;
        let num = (section.sh_size as u64) / entry_size;
        let mut data: [u8; 0xC] = [0; 0xC];
        let mut entries: Vec<RelaEntry> = Vec::with_capacity(num as usize);

        for n in 0..num {
            let pos = offset + n * entry_size;
            if let Err(e) = elf.seek(SeekFrom::Start(pos)) {
                return Err(e.to_string());
            }
            match elf.read(&mut data) {
                Ok(n) => if n != 0xC { return Err(String::from("invalid length")); },
                Err(e) => return Err(e.to_string()),
            };
            // parse data
            let mut cursor = &data[..];
            let r_offset = cursor.get_u32_le();
            let r_info = cursor.get_u32_le();
            let r_addend = cursor.get_i32_le();
            entries.push(RelaEntry { r_offset, r_info, r_addend });
        }

        Ok(RelaTable {
            section: section.clone(),
            data: entries,
        })
    }
}

#[derive(Debug, Default)]
pub struct ProgramHeader {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,
}

#[derive(Debug, Default)]
pub struct ProgramTable {
    // ELFHeader.e_phoff
    pub offset: u32,
    // ELFHeader.e_phentsize
    pub size: u16,
    // ELFHeader.e_phnum
    pub num: u16,
    pub programs: Vec<ProgramHeader>,
}

impl ProgramTable {
    pub fn new() -> ProgramTable {
        Self::default()
    }

    pub fn load<T>(elf: &mut T, ehdr: &ELFHeader) -> Result<ProgramTable, String>
        where T: Read + Seek
    {
        // check e_phentsize, min size is 0x20
        if ehdr.e_phentsize < 0x20 {
            return Err(String::from("invalid e_shentsize"));
        }
        let num = ehdr.e_phnum as u64;
        let size = ehdr.e_phentsize as u64;
        let offset = ehdr.e_phoff as u64;

        let mut data: [u8; 0x20] = [0; 0x20];
        let mut programs: Vec<ProgramHeader> = Vec::with_capacity(num as usize);

        for i in 0..num {
            // read data
            let pos: u64 = offset + i * size;
            if let Err(e) = elf.seek(std::io::SeekFrom::Start(pos)) {
                return Err(e.to_string());
            }
            match elf.read(&mut data) {
                Ok(n) => if n != 0x20 { return Err(String::from("invalid length")) },
                Err(e) => return Err(e.to_string()),
            };
            // parse section header
            let mut cursor = &data[..];
            let p_type: u32 = cursor.get_u32_le();
            let p_offset: u32 = cursor.get_u32_le();
            let p_vaddr: u32 = cursor.get_u32_le();
            let p_paddr: u32 = cursor.get_u32_le();
            let p_filesz: u32 = cursor.get_u32_le();
            let p_memsz: u32 = cursor.get_u32_le();
            let p_flags: u32 = cursor.get_u32_le();
            let p_align: u32 = cursor.get_u32_le();
            programs.push(ProgramHeader {
                p_type, p_offset, p_vaddr, p_paddr,
                p_filesz, p_memsz, p_flags, p_align,
            });
        }

        Ok(ProgramTable {
            programs,
            offset: ehdr.e_phoff,
            size: ehdr.e_phentsize,
            num: ehdr.e_phnum,
        })
    }
}
