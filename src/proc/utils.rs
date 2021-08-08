use std::{
    fs::File,
    io::BufReader,
    io::prelude::*
};


pub fn package_pid(package: &str) -> Result<u64, String> {
    // get pid
    let pid = match std::process::Command::new("pidof").arg(package).output() {
        Ok(output) => output,
        Err(e) => return Err(e.to_string()),
    };
    // if pidof failed, -1 is return
    if !pid.status.success() {
        return Err(format!("{} not running or not exist", package));
    }
    let mut pid = match String::from_utf8(pid.stdout) {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    };
    // remove newline of command output
    if pid.ends_with('\n') {
        pid.pop();
    }
    // parse pid to integer
    let pid = match pid.parse::<u64>() {
        Ok(p) => p,
        Err(e) => return Err(e.to_string()),
    };

    Ok(pid)
}

pub fn command_line(pid: u64) -> Result<Vec<String>, String> {
    let path = format!("/proc/{}/cmdline", pid);
    let cmdline = match File::open(path) {
        Ok(f) => BufReader::new(f),
        Err(e) => return Err(e.to_string()),
    };

    let mut result: Vec<String> = Vec::new();
    for s in cmdline.split(b'\0') {
        let s = match s {
            Ok(raw) => String::from_utf8(raw).expect("bad utf8"),
            Err(e) => return Err(e.to_string()),
        };
        // cmdline maybe end with '\0\0', so last iter element is empty, skip it
        if !s.is_empty() {
            result.push(s);
        }
    }

    Ok(result)
}
