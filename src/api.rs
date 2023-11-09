use crate::{
    output,
    types::{self, Arguments, Config},
    utils,
};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

fn generate_prompt(prompt: &str, args: &Arguments) -> String {
    match &args.translate {
        Some(expr) => {
            if expr == "fr" {
                return format!("{}{}", "Translate this sentence in french : ", prompt);
            } else if expr == "en" {
                return format!("{}{}", "Translate this sentence in english : ", prompt);
            } else {
                return format!("{}{} : {}", "Translate this sentence in ", expr, prompt);
            }
        }
        None => (),
    }
    match &args.correct {
        Some(expr) => {
            if expr == "fr" {
                return format!("{}{}", "Correct this sentence in french: ", prompt);
            } else if expr == "en" {
                return format!("{}{}", "Correct this sentence in english: ", prompt);
            } else {
                return format!("{}{} : {}", "Correct this sentence in ", expr, prompt);
            }
        }
        None => (),
    }
    String::from(prompt)
}

#[tokio::main]
pub async fn gpt_completion(
    prompt: String,
    config: &Config,
    args: &Arguments,
) -> Option<types::APIResponse> {
    //Create message to send
    let prompt = types::Message {
        role: "user".into(),
        content: generate_prompt(&prompt, args),
    };
    let mut model = "gpt-3.5-turbo-1106";
    if args.gpt_4 {
        model = "gpt-4-1106-preview"
    }
    //Create JSON for post request
    let json_body = types::Request {
        model: model.to_string(),
        messages: vec![prompt],
        temperature: config.temperature,
        max_tokens: config.max_tokens,
        top_p: config.top_p,
        presence_penalty: config.presence_penalty,
        frequency_penalty: config.frequency_penalty,
    };
    if args.config {
        output::print_config(&json_body)
    }

    //Create client to make api request
    let client = reqwest::Client::new();

    //Sending post request
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header(
            AUTHORIZATION,
            "Bearer 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",
        )
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&json_body).unwrap())
        .send()
        .await
        .unwrap_or_else(|_| {
            utils::exit("Error during POST request", true);
            panic!("")
        });

    //Parsing response form OpenAI
    let mut api_response: Option<types::APIResponse> = None;
    match response.status() {
        reqwest::StatusCode::OK => {
            let response = response.text().await.unwrap_or_else(|_| {
                utils::exit(
                    "Error during response parsing (getting response body)",
                    true,
                );
                panic!("")
            });
            let response =
                serde_json::from_str::<types::APIResponse>(&response).unwrap_or_else(|_| {
                    utils::exit(
                        "Error during response parsing (transforming txt to struct)",
                        true,
                    );
                    panic!("")
                });
            api_response = Some(response);
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        _ => {
            utils::exit("Uh oh! Something unexpected happened", true);
        }
    }
    api_response
}
