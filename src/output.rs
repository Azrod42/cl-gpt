use crate::color::change_color;
use crate::types::{Arguments, Config, Request};

use crate::{format, types::APIResponse};
pub fn print_avalible_colors() {
    println!("\u{001b}[0;31m\nError: Unknow color please use one of:");
    print!("\u{001b}[0;31m{} \u{001b}[0m default ", "red");
    print!("\u{001b}[0;32m{} ", "green");
    print!("\u{001b}[0;33m{} ", "orange");
    print!("\u{001b}[0;34m{} ", "blue");
    print!("\u{001b}[0;35m{} ", "magenta");
    print!("\u{001b}[0;36m{} ", "cyan");
    print!("\u{001b}[0;37m{}\n", "white");
}

pub fn print_config(request: &Request) {
    change_color("red");
    println!("CONFIG:");
    change_color("blue");
    println!(
        "model: {},\ntemperature: {},\ntop_p: {},\nmax_token: {}\nfrequency-penalty: {}\npresence-penalty: {}\n",
        request.model, request.temperature, request.top_p, request.max_tokens, request.frequency_penalty, request.presence_penalty
    );
}
pub fn print_info(response: &APIResponse, config: &Config) {
    change_color("red");
    println!("INFO:");
    change_color("blue");
    println!(
            "model: {},\nprompt: {} tokens,\nresponse: {} tokens,\ntotal: {} tokens,\nlifetime: {} tokens {:.4} euro\n",
            response.model,
            response.usage.prompt_tokens,
            response.usage.completion_tokens,
            response.usage.total_tokens,
            config.total_tokens,
            ((config.total_tokens as f32 / 1000.0) * 0.01)
        );
}

pub fn print_response(response: APIResponse, clap: &Arguments, config: &mut Config) {
    if response.usage.total_tokens != 0 {
        config.total_tokens = config.total_tokens + response.usage.total_tokens;
    }
    if clap.info {
        print_info(&response, config)
    }
    change_color(&config.color);
    let gpt_message = response.choices[response.choices.len() - 1]
        .message
        .content
        .clone();
    let gpt_message = format::md_code_format(gpt_message, &config);

    println!("{}", gpt_message);
}
