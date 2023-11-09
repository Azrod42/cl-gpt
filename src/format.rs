use crate::{color::change_color, color::get_color, types::Config, utils};

pub fn md_code_format(mut gpt_message: String, config: &Config) -> String {
    while gpt_message.contains("```") {
        let start = gpt_message.find("```").unwrap_or(0);
        let end = gpt_message[start..].to_string().find("\n").unwrap_or(0);
        let end = start + end;
        if start == 0 || end == 0 {
            utils::exit("md_code_format parsing error", true)
        }
        gpt_message = format!(
            "{}{}{}",
            &gpt_message[0..start],
            get_color(&config.code_color),
            &gpt_message[end..]
        );

        let start = gpt_message.find("```").unwrap_or(0);
        let end = start + 3;
        change_color(&config.color);
        gpt_message = format!(
            "{}{}{}",
            &gpt_message[0..start],
            get_color(&config.color),
            &gpt_message[end..]
        );
    }
    gpt_message
}
