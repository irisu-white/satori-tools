use satori::proc::maps;
use satori::proc::mem;
use satori::proc::utils;

use clap::{Arg, App, SubCommand};


fn main() -> Result<(), String> {
    let matches = App::new("Satori Watcher")
        .about("Read, write and parse /proc info")
        .version("0.0.1")
        .author("irisu whte <irisu@uprprc.net>")
        .subcommand(SubCommand::with_name("maps")
            .arg(Arg::with_name("filte")
                .short("f").long("filte")
                .value_name("FILTER")
                .help("filte pathname of page")
                .takes_value(true))
            .arg(Arg::with_name("range")
                .short("r").long("range")
                .value_name("ADDRESS")
                .help("only show page range in address")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("cmdline"))
        .subcommand(SubCommand::with_name("dump-library")
            .arg(Arg::with_name("library")
                .required(true)
                .help("target library name"))
            .arg(Arg::with_name("output")
                .required(true)
                .help("output path")))
        .arg(Arg::with_name("package")
            .required(true)
            .index(1)
            .help("the pacakge name of App"))
        .get_matches();
    
    // unwrap package name
    let package_name = matches.value_of("package").unwrap();
    let pid = utils::package_pid(package_name)?;

    // subcommand: maps info
    if let Some(cmd_maps) = matches.subcommand_matches("maps") {
        let filte = cmd_maps.value_of("filte");
        let range = cmd_maps.value_of("range");
        // run maps
        let mut maps = maps::read_maps(pid)?;
        let all_count = maps.len();
        // begin print
        println!("> list maps of {}", package_name);
        // apply filte and range
        if let Some(filte_name) = filte {
            println!("> filte pathname {}", filte_name);
            maps = maps.into_iter().filter(|v| v.match_name(filte_name)).collect();
        }
        if let Some(range_address) = range {
            // TODO: parse hex-string
            let address: u64 = range_address.parse().unwrap();
            println!("> range address {}", address);
            maps = maps.into_iter().filter(|v| v.match_range(address)).collect();
        }
        // show all
        for info in &maps {
            println!("{}", info);
        }
        if maps.len() == all_count {
            println!("page count {}", all_count);
        }
        else {
            println!("page count {}/{}(current/all)", maps.len(), all_count);
        }
    }

    // subcommand: process arguments
    if matches.subcommand_matches("cmdline").is_some() {
        let cmdline = utils::command_line(pid)?;
        // show all args
        println!("{}", cmdline.join(" "));
    }

    if let Some(dump_library) = matches.subcommand_matches("dump-library") {
        let maps = maps::read_maps(pid)?;
        let library_name = dump_library.value_of("library").unwrap();
        let output_path = dump_library.value_of("output").unwrap();
        match mem::dump_library(pid, &maps, library_name, output_path) {
            Ok(..) => { },
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(())
}
