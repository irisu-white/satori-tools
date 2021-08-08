use std::{
    fmt, fmt::Write,
    fs::File,
    io::BufReader,
    io::prelude::*,
};


#[derive(Clone)]
pub struct PageInfo {
    pub begin: u64,
    pub end: u64,
    pub perms: Perms,
    pub offset: u64,
    pub dev: Dev,
    pub inode: u32,
    pub pathname: String,
}

#[derive(Clone)]
pub struct Dev {
    pub major: u32,
    pub minor: u32,
}

#[derive(Clone)]
pub struct Perms {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub shared: bool,
    pub private: bool,
}

impl PageInfo {
    pub fn match_range(&self, address: u64) -> bool {
        address >= self.begin && address < self.end
    }

    pub fn match_name(&self, name: &str) -> bool {
        match &self.pathname.find(name) {
            Some(..) => true,
            None => false,
        }
    }
}

impl fmt::Display for PageInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name: &str = if self.pathname.is_empty() {
            "<anonymous>"
        }
        else {
            &self.pathname
        };
        write!(f, "{:#x} {:#x} {} {:#x} {} {} {}", self.begin, self.end,
            self.perms, self.offset, self.dev, self.inode, name)
    }
}

impl fmt::Display for Dev {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02x}:{:02x}", self.major, self.minor)
    }
}

impl fmt::Display for Perms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char(if self.read { 'r' } else { '-' })?;
        f.write_char(if self.write { 'w' } else { '-' })?;
        f.write_char(if self.execute { 'x' } else { '-' })?;
        f.write_char(if self.shared { 's' } else if self.private { 'p' } else { '-' })?;
        Ok(())
    }
}

pub fn read_maps(pid: u64) -> Result<Vec<PageInfo>, String> {
    // open maps
    let path = format!("/proc/{}/maps", pid);
    let maps = match File::open(&path) {
        Ok(f) => BufReader::new(f),
        Err(e) => return Err(e.to_string()),
    };

    // resule
    let mut result: Vec<PageInfo> = Vec::new();

    // read line by line
    for line in maps.lines() {
        let line_str = match line {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        };

        let data: Vec<&str> = line_str.splitn(6, ' ').collect();
        // address
        let address = data[0];
        let mid = address.find('-').expect(address);
        let begin = u64::from_str_radix(&address[..mid], 16).expect(address);
        let end = u64::from_str_radix(&address[(mid+1)..], 16).expect(address);
        // perms
        let perms = data[1].as_bytes();
        let read = perms[0] == b'r';
        let write = perms[1] == b'w';
        let execute = perms[2] == b'x';
        let shared = perms[3] == b's';
        let private = perms[3] == b'p';
        let perms = Perms{ read, write, execute, shared, private};
        // offset
        let offset = u64::from_str_radix(data[2], 16).expect(data[2]);
        // dev
        let dev_str = data[3];
        let mid = dev_str.find(':').expect(dev_str);
        let major = u32::from_str_radix(&dev_str[..mid], 16).expect(dev_str);
        let minor = u32::from_str_radix(&dev_str[(mid+1)..], 16).expect(dev_str);
        let dev = Dev{ major, minor };
        // inode
        let inode: u32 = data[4].parse().expect(data[4]);
        // pathname
        let mut pathname = String::new();
        for (i, c) in data[5].chars().enumerate() {
            if c == ' ' {
                continue;
            }
            pathname = String::from(&data[5][i..]);
            break;
        }

        // collect them
        result.push(PageInfo{ begin, end, perms, offset, dev, inode, pathname });
    }

    Ok(result)
}
