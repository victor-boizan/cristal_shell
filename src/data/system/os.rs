use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub enum OS {
    //Represent the type of operating system we are running
    NixOS,
    Unknown,
}

impl OS {
    pub fn current() -> Self {
        if Path::new("/etc/os-release").exists() {
            let file = File::open("/etc/os-release").ok().unwrap();
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line.ok().unwrap();
                if let Some((key, value)) = line.split_once('=') {
                    if key.trim() == "ID" {
                        return Self::from_id(value);
                    }
                }
            }
        }
        Self::Unknown
    }

    pub fn get_icon(&self) -> String {
        match self {
            Self::NixOS => String::from(""),
            Self::Unknown => String::from("?"),
        }
    }

    fn from_id(id: &str) -> Self {
        //should match the ID field in /etc/os-release when exists
        match id {
            "nixos" => Self::NixOS,
            _ => Self::Unknown,
        }
    }
}
