use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Default, Debug, Clone)]
pub struct Arguments {
    pub arg: Vec<String>,

    #[clap(short, long, default_value_t = false)]
    pub info: bool,

    #[clap(long)]
    pub color: Option<String>,

    #[clap(long)]
    pub code_color: Option<String>,

    #[clap(short, long)]
    pub max_tokens: Option<i16>,

    #[clap(long)]
    pub temperature: Option<f32>,

    #[clap(long)]
    pub top_p: Option<f32>,

    #[clap(long)]
    pub frequency_penalty: Option<f32>,

    #[clap(long)]
    pub presence_penalty: Option<f32>,

    #[clap(long, default_value_t = false)]
    pub config: bool,

    #[clap(short, long, default_value_t = false)]
    pub translate: bool,

    #[clap(short, long, default_value_t = false)]
    pub correct: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub max_tokens: i16,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choices {
    pub index: i32,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse {
    pub id: String,
    pub object: String,
    pub created: i32,
    pub model: String,
    pub choices: Vec<Choices>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub color: String,
    pub code_color: String,
    pub total_tokens: i32,
    pub max_tokens: i16,
    pub temperature: f32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
}
