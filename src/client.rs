use std::option::Option;
use std::process::Command;
use std::str;
use std::string::String;
use std::vec::Vec;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::result::Result;

pub fn raw_memory() -> Result<Vec<u8>, &'static str> {
    match pid() {
        Some(pid) => match mem(pid) {
            Some(mem) => Ok(mem),
            None => {
                error!("Couldn't read memory");
                Err("Couldn't read memory")
            }
        },
        None => {
            error!("Tibia client not found!");
            Err("Couldn't read memory")
        }
    }
}

fn mem(pid: u32) -> Option<Vec<u8>> {
    if let Some(heap_region) = heap(pid) {
        let mut mem_file = File::open(format!("/proc/{}/mem", pid)).unwrap();
        let mut file = BufReader::new(&mem_file);
        file.seek(SeekFrom::Start(heap_region.start)).unwrap();
        let mut chunk = file.take(heap_region.end - heap_region.start);
        let mut mem = vec![];
        chunk.read_to_end(&mut mem).unwrap();
        return Some(mem);
    }
    None
}

fn pid() -> Option<u32> {
    let out = Command::new("ps").arg("ax").output().unwrap();
    let stdout = String::from(str::from_utf8(&out.stdout).unwrap());
    let processes: Vec<&str> = stdout.split("\n").collect();
    let tibia_process = processes.iter().find(|row| 
        row.find("Tibia/bin/client").is_some()
        && row.find("--package-is-up-to-date").is_none()
    );

    match tibia_process {
        Some(process) => {
            let process_entry = process.split(" ").collect::<Vec<&str>>();
            if process_entry.len() < 1 {
                debug!("invalid  ps output");
                return None;
            }
            Some(match process_entry[0].parse() {
                Ok(p_id) => p_id,
                Err(_) => process_entry[1].parse().unwrap(),
            })
        }
        None => None,
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MemoryRegion {
    start: u64,
    end: u64,
}

fn heap(pid: u32) -> Option<MemoryRegion> {
    let f = File::open(format!("/proc/{}/maps", pid)).unwrap();
    let file = BufReader::new(&f);
    for raw_line in file.lines() {
        let line = raw_line.unwrap();
        if let Some(_) = line.find("heap") {
            // 1234-1235 rw-p 00000000 00:00 0    [heap]
            let region = line.split(" ").collect::<Vec<&str>>()[0]
                .split("-")
                .collect::<Vec<&str>>();
            if region.len() < 2 {
                debug!("invalid memory map format");
                return None;
            }
            return Some(MemoryRegion {
                start: u64::from_str_radix(&region[0], 16).unwrap(),
                end: u64::from_str_radix(&region[1], 16).unwrap(),
            });
        }
    }
    None
}
