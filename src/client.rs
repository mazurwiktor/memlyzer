use std::option::Option;
use std::process::Command;
use std::str;
use std::string::String;
use std::vec::Vec;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::result::Result;
use libc::{c_void, iovec, pid_t, process_vm_readv};
use std::io;

type Pid = pid_t;

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
        let mut buffer = vec![0;  (heap_region.end - heap_region.start) as usize];
        match copy_address(pid as Pid, heap_region.start as usize, &mut buffer) {
            Ok(_) => return Some(buffer),
            Err(e) => panic!(e)
        }
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
            if process_entry.len() <= 1 {
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
            if region.len() <= 1 {
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


fn copy_address(pid: Pid, addr: usize, buf: &mut [u8]) -> io::Result<()> {
    let local_iov = iovec {
        iov_base: buf.as_mut_ptr() as *mut c_void,
        iov_len: buf.len(),
    };
    let remote_iov = iovec {
        iov_base: addr as *mut c_void,
        iov_len: buf.len(),
    };
    let result = unsafe { process_vm_readv(pid, &local_iov, 1, &remote_iov, 1, 0) };
    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
