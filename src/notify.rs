use std::process::Command;
use std::str;

pub fn notify(title: &str, msg: &str) {
    let who = Who::new();
    Command::new("sudo")
        .arg(format!("-u{}", &who.username))
        .arg(format!("DISPLAY={}", who.display))
        .arg(format!(
            "DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/{}/bus",
            who.uid
        ))
        .arg("notify-send")
        .args(&[title, msg])
        .output()
        .unwrap();
}

#[derive(Debug)]
struct Who {
    pub username: String,
    pub display: String,
    pub uid: u32,
}

impl Who {
    pub fn new() -> Self {
        let out = Command::new("who").output().unwrap();
        let args = str::from_utf8(&out.stdout)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();
        let username = String::from(args[0]);
        Who {
            display: String::from(args[1]),
            uid: str::from_utf8(
                &Command::new("id")
                    .arg("-u")
                    .arg(&username)
                    .output()
                    .unwrap()
                    .stdout,
            ).unwrap()
                .trim()
                .parse()
                .unwrap(),
            username,
        }
    }
}
