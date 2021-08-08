use satori::proc::maps;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufWriter, prelude::*};
use clap::{Arg, App};


fn main() {
    let matches = App::new("Process Memory dumper")
        .about("Dump memory of process, skip unreadable pages")
        .version("0.1.0")
        .author("irisu white <irisu@uprprc.net>")
        .arg(Arg::with_name("pid")
            .short("p").long("pid")
            .value_name("PID")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("begin")
            .short("b").long("begin")
            .value_name("address")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("end")
            .short("e").long("end")
            .value_name("address")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o").long("output")
            .value_name("file path")
            .required(true)
            .takes_value(true))
        .get_matches();
    
    // get command arguments
    let pid= matches.value_of("pid").unwrap();
    let begin_str= matches.value_of("begin").unwrap();
    let end_str= matches.value_of("end").unwrap();
    let output= matches.value_of("output").unwrap();
    // parse begin address
    let begin: u64;
    if begin_str.starts_with("0x") || begin_str.starts_with("0X") {
        begin = u64::from_str_radix(&begin_str[2..], 16)
            .expect("invalid begin address");
    }
    else {
        begin = begin_str.parse::<u64>()
            .expect("invalid begin address");
    }
    let pid = pid.parse::<u64>().expect("invalid pid");
    // parse end address
    let end: u64;
    if end_str.starts_with("0x") || end_str.starts_with("0X") {
        end = u64::from_str_radix(&end_str[2..], 16)
            .expect("invalid begin address");
    }
    else {
        end = end_str.parse::<u64>()
            .expect("invalid begin address");
    }
    // run
    match mem_dump(pid, begin, end, output) {
        Ok(n) => println!("success: dump {} pages", n),
        Err(e) => eprintln!("Error: {}", e),
    };
}

fn mem_dump(pid: u64, begin: u64, end: u64, output: &str) -> Result<i32, String> {
    // check range
    if begin >= end {
        return Err(String::from("Invalid begin-end range"));
    }

    // read process maps
    let maps = maps::read_maps(pid)?;

    // find fist page
    let mut first: Option<usize> = None;
    for (idx, page) in maps.iter().enumerate() {
        if begin >= page.begin && begin < page.end {
            first = Some(idx);
            break;
        }
    }
    if first.is_none() {
        return Err(String::from("begin address out of range"));
    }

    // open file
    let mem_path = format!("/proc/{}/mem", pid);
    let mut mem = match File::open(mem_path) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };
    let mut out = match File::create(output) {
        Ok(f) => BufWriter::new(f),
        Err(e) => return Err(e.to_string()),
    };

    // dump data
    let mut count = 0;
    for page in maps.iter().skip(first.unwrap()) {
        // dump finish
        if page.begin >= end {
            break;
        }
        // skip unreadable pages
        if page.perms.read == false {
            continue;
        }
        // copy data
        let page_begin = max(page.begin, begin);
        let page_end = min(page.end,end);
        let mut buf: Vec<u8> = Vec::new();
        buf.resize((page_end - page_begin) as usize, 0);

        mem.seek(std::io::SeekFrom::Start(page_begin)).unwrap();
        out.seek(std::io::SeekFrom::Start(page_begin - begin)).unwrap();
        match mem.read(&mut buf) {
            Ok(_) => { },
            Err(e) => return Err(e.to_string()),
        };
        match out.write_all(&mut buf) {
            Ok(_) => { },
            Err(e) => return Err(e.to_string()),
        };
        count += 1;
    }

    Ok(count)
}
