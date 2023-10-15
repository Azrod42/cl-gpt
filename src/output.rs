use crate::color::change_color;
use crate::types::{Arguments, Config, Request};

use crate::{format, types::APIResponse};

pub fn print_config(request: &Request) {
    change_color(&String::from("red"));
    println!(
        "CONFIG:\nmodel: {},\ntemperature: {},\ntop_p: {}",
        request.model, request.temperature, request.top_p
    );
}

pub fn print_response(response: APIResponse, clap: &Arguments, config: &mut Config) {
    if response.usage.total_tokens != 0 {
        config.total_tokens = config.total_tokens + response.usage.total_tokens;
    }
    if clap.info {
        print!("\u{001b}[0;91m");
        println!(
            "INFO:\n model: {},\n prompt: {} tokens,\n response: {} tokens,\n total: {} tokens,\n lifetime: {} tokens\n",
            response.model,
            response.usage.prompt_tokens,
            response.usage.completion_tokens,
            response.usage.total_tokens,
            config.total_tokens
        );
    }
    // change_color(&String::from("default"));
    // println!("==================\n\tGPT\n==================");
    change_color(&config.color);
    let gpt_message = response.choices[response.choices.len() - 1]
        .message
        .content
        .clone();
    let gpt_message = format::md_code_format(gpt_message, &config);

    println!("{}", gpt_message);
}
