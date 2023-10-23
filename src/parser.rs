use std::env;

use crate::files::{create_conf_file, delete_conf_file, read_conf_file};
use crate::types::{Arguments, Config};
use crate::utils;

fn check_color_input(arg: Arguments, config: &mut Config) {
    let accepted_color = vec![
        "red", "green", "orange", "blue", "magenta", "cyan", "white", "default",
    ];
    match arg.color {
        Some(color) => {
            if accepted_color.iter().any(|&i| i == color) {
                config.color = color;
            } else {
                println!("\u{001b}[0;31m\nError: Unknow color please use one of:");
                print!("\u{001b}[0;31m{} \u{001b}[0m default ", "red");
                print!("\u{001b}[0;32m{} ", "green");
                print!("\u{001b}[0;33m{} ", "orange");
                print!("\u{001b}[0;34m{} ", "blue");
                print!("\u{001b}[0;35m{} ", "magenta");
                print!("\u{001b}[0;36m{} ", "cyan");
                print!("\u{001b}[0;37m{}\n", "white");
                std::process::exit(1);
            }
        }
        None => {}
    }

    match arg.code_color {
        Some(color) => {
            if accepted_color.iter().any(|&i| i == color) {
                config.code_color = color;
            } else {
                println!("\u{001b}[0;31m\nError: Unknow code_color please use one of:");
                print!("\u{001b}[0;31m{} \u{001b}[0m default ", "red");
                print!("\u{001b}[0;32m{} ", "green");
                print!("\u{001b}[0;33m{} ", "orange");
                print!("\u{001b}[0;34m{} ", "blue");
                print!("\u{001b}[0;35m{} ", "magenta");
                print!("\u{001b}[0;36m{} ", "cyan");
                print!("\u{001b}[0;37m{}\n", "white");
                std::process::exit(1);
            }
        }
        None => {}
    }
}

fn get_conf_file() -> Config {
    //Check if HOME env is set if not return error
    let user = env::var("HOME").unwrap_or_else(|_| {
        utils::exit(
            "No HOME path set please use 'export HOME=/home/YOUR_USER_NAME'",
            true,
        );
        String::from("")
    });

    //Read config file
    let config_option = read_conf_file(&user);

    //Create default config
    let mut config: Config = Config {
        color: "default".into(),
        code_color: "default".into(),
        total_tokens: 0,
        max_tokens: 2048,
        temperature: 1.0,
        top_p: 1.0,
        presence_penalty: 0.0,
        frequency_penalty: 0.0,
    };
    //Pars conf file if error inside > recreate one
    match config_option {
        Some(conf) => {
            config = serde_json::from_str::<Config>(&conf).unwrap_or_else(|_| {
                utils::exit("Error during parsing conf file. Deleting file", false);
                delete_conf_file(&user);
                std::process::exit(1);
            });
        }
        None => create_conf_file(&config, &user),
    }
    config
}

fn check_max_token(user_input: Option<i16>, config: &mut Config) {
    match user_input {
        Some(user_input) => {
            if user_input < 1 || user_input > 4096 {
                utils::exit(
                    "Invalid max-token input (range: 1 - 4096, default: 2048)",
                    true,
                )
            } else {
                config.max_tokens = user_input;
            }
        }
        None => {}
    }
}
fn check_temperature(user_input: Option<f32>, config: &mut Config) {
    match user_input {
        Some(user_input) => {
            if user_input < 0.0 || user_input > 2.0 {
                utils::exit(
                    "Invalid max-token input (range: 0.0 - 2.0, default: 1.0)",
                    true,
                )
            } else {
                config.temperature = user_input;
            }
        }
        None => {}
    }
}

fn check_top_p(user_input: Option<f32>, config: &mut Config) {
    match user_input {
        Some(user_input) => {
            if user_input < 0.0 || user_input > 1.0 {
                utils::exit("Invalid top-p input (range: 0.0 - 1.0, default: 1.0)", true)
            } else {
                config.top_p = user_input;
            }
        }
        None => {}
    }
}
fn check_presence_penalty(user_input: Option<f32>, config: &mut Config) {
    match user_input {
        Some(user_input) => {
            if user_input < 0.0 || user_input > 2.0 {
                utils::exit(
                    "Invalid presence penalty input (range: 0.0 - 2.0, default: 0.0)",
                    true,
                )
            } else {
                config.presence_penalty = user_input;
            }
        }
        None => {}
    }
}
fn check_frequency_penalty(user_input: Option<f32>, config: &mut Config) {
    match user_input {
        Some(user_input) => {
            if user_input < 0.0 || user_input > 2.0 {
                utils::exit(
                    "Invalid frequency penalty input (range: 0.0 - 2.0, default: 0.0)",
                    true,
                )
            } else {
                config.frequency_penalty = user_input;
            }
        }
        None => {}
    }
}

fn check_special_flag(arg: &Arguments) {
    if arg.translate && arg.correct {
        utils::exit("Please use only one special flag", true)
    }
}

pub fn pars_argument(arg: Arguments) -> Config {
    let mut config = get_conf_file();
    check_special_flag(&arg);
    check_max_token(arg.max_tokens, &mut config);
    check_temperature(arg.temperature, &mut config);
    check_top_p(arg.top_p, &mut config);
    check_presence_penalty(arg.presence_penalty, &mut config);
    check_frequency_penalty(arg.frequency_penalty, &mut config);
    check_color_input(arg, &mut config);
    config
}
