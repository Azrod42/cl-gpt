use crate::color::{change_color, get_color};

pub fn exit(error: &str, exit: bool) {
    change_color(&String::from("red"));
    println!("Error:{} {}", get_color(&String::from("orange")), error);
    if exit {
        std::process::exit(1);
    }
}
