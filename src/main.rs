use clap::Parser;
use files::{create_conf_file, delete_conf_file};
use std::env;
use utils::exit;

mod api;
mod color;
mod files;
mod format;
mod output;
mod parser;
mod types;
mod utils;

fn main() {
    let raw_args = types::Arguments::parse();
    let mut config = parser::pars_argument(raw_args.clone());
    let prompt = raw_args.arg.join(" ");

    if prompt.len() > 1 {
        let api_response = api::gpt_completion(prompt, &config, &raw_args);

        match api_response {
            Some(response) => output::print_response(response, &raw_args, &mut config),
            None => exit(
                "In src/api.rs replace 'YOUR_API_KEY' with a valid OpenAI api key",
                true,
            ),
        }
    }
    delete_conf_file(&env::var("HOME").unwrap());
    create_conf_file(&config, &env::var("HOME").unwrap());
}
