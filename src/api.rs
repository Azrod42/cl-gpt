use crate::{
    output,
    types::{self, Arguments, Config},
    utils,
};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

fn generate_prompt(prompt: &str, args: &Arguments) -> String {
    if args.translate {
        return format!("{}{}", "Traduis cette phrase en anglais : ", prompt);
    } else if args.correct {
        return format!("{}{}", "Corrige cette phrase : ", prompt);
    } else {
        String::from(prompt)
    }
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
    //Create JSON for post request
    let json_body = types::Request {
        model: "gpt-3.5-turbo".into(),
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
            "Bearer YOUR_API_TOKEN",
        )
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&json_body).unwrap())
        .send()
        .await
        .unwrap_or_else(|_| {
            utils::exit("Error during POST request", true);
            panic!("")
        });

    //Paring response form OpenAI
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
