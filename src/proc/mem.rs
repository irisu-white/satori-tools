use std::{
    fs::File,
    io, io::prelude::*,
};
use crate::proc::maps::PageInfo;


pub fn dump_pages(pid: u64, mut target: Vec<PageInfo>, output_path: &str) -> io::Result<()> {
    // skip empty
    if target.len() == 0 {
        return Ok(());
    }
    // sort by PageInfo.begin
    target.sort_by(|a, b| a.begin.cmp(&b.begin));
    println!("dump {} pages", target.len());

    let path = format!("/proc/{}/mem", pid);
    let mut memory = File::open(path)?;
    let mut output = File::create(output_path)?;

    let start = target[0].begin;
    let mut buf: Vec<u8> = Vec::new();

    for info in target {
        let size = (info.end - info.begin) as usize;
        let offset = info.begin - start;
        buf.resize(size, 0);
        memory.seek(std::io::SeekFrom::Start(info.begin))?;
        memory.read(&mut buf)?;
        output.seek(std::io::SeekFrom::Start(offset))?;
        output.write(&buf)?;
    }

    Ok(())
}

pub fn dump_library(pid: u64, maps: &Vec<PageInfo>, name: &str, output_path: &str) -> io::Result<()> {
    let mut target: Vec<PageInfo> = Vec::new();
    for info in maps {
        if info.pathname == name  {
            target.push(info.clone());
        }
    }

    dump_pages(pid, target, output_path)
}
