use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use crate::types::Config;
use crate::utils;

pub fn read_conf_file(user: &String) -> Option<String> {
    let filename = format!("{user}/.config/gpt.conf");
    match File::open(filename.clone()) {
        Err(_) => None, //println!("failed to open file '{}': {}", filename, why),
        Ok(mut f) => {
            let mut data = String::new();
            match f.read_to_string(&mut data) {
                Err(_) => utils::exit("failed to read", true),
                Ok(_) => {}
            };
            Some(data)
        }
    }
}
pub fn create_conf_file(default_conf: &Config, user: &String) {
    let path = format!("{user}/.config/gpt.conf");
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    let default_conf = serde_json::to_string(&default_conf).unwrap();
    f.write_all(default_conf.as_ref()).unwrap();
}

pub fn delete_conf_file(user: &String) {
    let path = format!("{user}/.config/gpt.conf");
    let _ = std::fs::remove_file(path);
}
