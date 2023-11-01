pub fn change_color(color: &str) {
    match color.as_ref() {
        "red" => print!("\u{001b}[0;31m"),
        "green" => print!("\u{001b}[0;32m"),
        "orange" => print!("\u{001b}[0;33m"),
        "blue" => print!("\u{001b}[0;34m"),
        "magenta" => print!("\u{001b}[0;35m"),
        "cyan" => print!("\u{001b}[0;36m"),
        "white" => print!("\u{001b}[0;37m"),
        &_ => print!("\u{001b}[0m"), //default term color
    }
}
pub fn get_color(color: &String) -> &str {
    match color.as_ref() {
        "red" => return "\u{001b}[0;31m",
        "green" => return "\u{001b}[0;32m",
        "orange" => return "\u{001b}[0;33m",
        "blue" => return "\u{001b}[0;34m",
        "magenta" => return "\u{001b}[0;35m",
        "cyan" => return "\u{001b}[0;36m",
        "white" => return "\u{001b}[0;37m",
        &_ => return "\u{001b}[0m", //default term color
    }
}
