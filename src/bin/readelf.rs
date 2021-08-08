use std::fs::File;
use std::io::BufReader;
use satori::elf::elf32::{
    ELFHeader,
    SectionTable,
    StringTable,
    //ProgramTable,
    //SymbolTable,
};


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file path>", args[0]);
        return;
    }

    let mut elf = match File::open(&args[1]) {
        Ok(f) => BufReader::new(f),
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // elf header
    let elf_header = match ELFHeader::load(&mut elf) {
        Ok(hdr) => hdr,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    println!("{:#x?}", elf_header);

    // section header
    let section_table = match SectionTable::load(&mut elf, &elf_header) {
        Ok(hdr) => hdr,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    //println!("{:#x?}", section_table);

    // format section header
    if let Err(e) = print_section_table(&mut elf, &section_table) {
        eprintln!("Error: {}", e);
    }

    // program header
    //let program_table = match ProgramTable::load(&mut elf, &elf_header) {
    //    Ok(phdr) => phdr,
    //    Err(e) => {
    //        eprintln!("Error: {}", e);
    //        return;
    //    }
    //};
    //println!("{:#x?}", program_table);

    // symbol table
    //for s in &section_table.data {
    //    if s.sh_type != elf_rs::value::SHT_SYMTAB {
    //        continue;
    //    }
    //    let symbol_table = match SymbolTable::load(&mut elf, s) {
    //        Ok(st) => st,
    //        Err(e) => {
    //            eprintln!("Error: {}", e);
    //            return;
    //        }
    //    };
    //    println!("{:#x?}", symbol_table);
    //    break;
    //}
}

#[allow(dead_code)]
pub fn print_section_table<T>(elf: &mut T, st: &SectionTable) -> Result<(), String>
    where T: std::io::Read + std::io::Seek
{
    let name_section = &st.data[st.name as usize];
    let name_table = match StringTable::load(elf, name_section) {
        Ok(table) => table,
        Err(e) => return Err(e),
    };

    for (idx, shdr) in st.data.iter().enumerate() {
        let name = name_table.get(shdr.sh_name as usize);
        // parse flag
        let flag = shdr.sh_flags;
        let mut flag_str = format!("{:#x}", flag);
        if flag != 0 {
            flag_str.push_str(" {");
            if (flag & 0x1) != 0 {
                flag_str.push_str(" WRITE");
            }
            if (flag & 0x2) != 0 {
                flag_str.push_str(" ALLOC");
            }
            if (flag & 0x4) != 0 {
                flag_str.push_str(" EXECINSTR");
            }
            if (flag & 0xF0000000) != 0 {
                flag_str.push_str(" MASKPROC");
            }
            if (flag & 0x0FFFFFF8) != 0 {
                flag_str.push_str(" OTHER");
            }
            flag_str.push_str(" }");
        }
        // match type
        let type_str  = match shdr.sh_type {
            0 => "SHT_NULL",
            1 => "SHT_PROGBITS",
            2 => "SHT_SYMTAB",
            3 => "SHT_STRTAB",
            4 => "SHT_RELA",
            5 => "SHT_HASH",
            6 => "SHT_DYNAMIC",
            7 => "SHT_NOTE",
            8 => "SHT_NOBITS",
            9 => "SHT_REL",
            10 => "SHT_SHLIB",
            11 => "SHT_DYNSYM",
            0x70000000 => "SHT_LOPROC",
            0x7fffffff => "SHT_HIPROC",
            0x80000000 => "SHT_LOUSER",
            0xffffffff => "SHT_HIUSER",
            _ => "INVALID",
        };
        // match link and info
        let link = match shdr.sh_link {
            0 => "SHN_UNDEF".to_string(),
            0xff00 => "SHN_LOPROC".to_string(),
            0xff1f => "SHN_HIPROC".to_string(),
            0xfff1 => "SHN_ABS".to_string(),
            0xfff2 => "SHN_COMMON".to_string(),
            0xffff => "SHN_HIRESERVE".to_string(),
            n => format!("{}", n),
        };
        let info = match shdr.sh_info {
            0 => "SHN_UNDEF".to_string(),
            0xff00 => "SHN_LOPROC".to_string(),
            0xff1f => "SHN_HIPROC".to_string(),
            0xfff1 => "SHN_ABS".to_string(),
            0xfff2 => "SHN_COMMON".to_string(),
            0xffff => "SHN_HIRESERVE".to_string(),
            n => format!("{}", n),
        };
        // format
        println!("section {} {{", idx);
        println!("\tname: {} ({:#x})", name, shdr.sh_name);
        println!("\ttype: {} ({:#x})", type_str, shdr.sh_type);
        println!("\tflags {}", flag_str);
        println!("\taddr: {:#x}", shdr.sh_addr);
        println!("\toffset: {:#x}", shdr.sh_offset);
        println!("\tsize: {:#x}", shdr.sh_size);
        println!("\tlink: {}", link);
        println!("\tinfo: {}", info);
        println!("\taddr align: {}", shdr.sh_addralign);
        println!("\tentry size: {:#x}", shdr.sh_entsize);
        println!("}}");
    }

    Ok(())
}
