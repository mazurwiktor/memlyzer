#[macro_use]
extern crate lazy_static;
extern crate config;
extern crate regex;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate libc;
mod client;
mod configuration;
mod loot;
mod memory;
mod message;
mod notify;

use simplelog::*;
use std::fs::File;
use std::thread;
use std::time;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("memlyzer.log").unwrap(),
        ),
    ]).unwrap();

    info!("Tibia memory analyzer started");
    let conf = configuration::Configuration::new();
    notify::notify("Tibia memory analyzer", "Successfully started");
    match client::raw_memory() {
        Ok(_) => info!("Memory fetched successfully..."),
        Err(e) => panic!(e),
    }
    info!("Checking messages from stack...");

    check_messages(|messages| {
        for msg in messages {
            let loot = loot::Loot::from(&msg).filter(&conf.loot_list);
            info!(
                "{} dropped : {}",
                loot.monster_name(),
                loot::Loot::from(&msg).looted_items()
            );
            if !loot.looted_items().is_empty() {
                notify::notify(
                    &format!("{}", loot.monster_name()),
                    &format!("{}", loot.looted_items()),
                );
            }
        }
    });
}

fn check_messages<F>(on_msg_change: F)
where
    F: Fn(Vec<String>),
{
    loop {
        if let Ok(old_memory) = client::raw_memory() {
            let old_snapshot = memory::Snapshot::from(&old_memory);
            loop {
                thread::sleep(time::Duration::from_millis(200));
                if let Ok(new_memory) = client::raw_memory() {
                    let new_snapshot = memory::Snapshot::from(&new_memory);
                    let diff = old_snapshot.diff(&new_snapshot);
                    if !diff.is_empty() {
                        on_msg_change(diff.messages);
                        break;
                    }
                } else {
                    thread::sleep(time::Duration::from_secs(5));
                }
            }
        }
    }
}
