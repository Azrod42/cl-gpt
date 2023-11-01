# cli-gpt build in rust

Command-line interface using ChatGPT

![gpt](https://i.postimg.cc/L4NXhDNh/Selection-008.png)

# Feature

- Command-Line Interface: Interact with ChatGPT from your terminal.
- Customize the Model: Take complete control over the AI's behavior by overriding the default temperature, top_p, frequency_penalty and presence_penalty values for each assistant.
- Track how you use the API and get detailed information about the number of tokens consumed and the associated costs.
- The personalized flag -t -c provides quick access to powerful translation and correction powered by artificial intelligence.
- Color customization: customize the dominant color of the responses and the color of the code.
- Memory of settings: allows to store and remember your ChatGPT configuration.

# Installation

Retrieve your API key here: https://platform.openai.com/account/api-keys and execute the following command by replacing YOUR_API_KEY.

```bash
export OPEN_AI_KEY=YOUR_API_KEY
```

Install rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

!!!RESTART YOUR SHELL!!!

## Automatic installation

(only works on Mac and Linux using a zshrc).

```bash
git clone https://github.com/Azrod42/cli-gpt.git && cd cli-gpt && sh install.sh
```

## Manual installation

Download project and go inside:

```bash
git clone https://github.com/Azrod42/cli-gpt.git && cd cli-gpt
```

Replace YOUR_API_KEY in src/api.rs by your api key:

```bash
perl -pi -e "s/YOUR_API_KEY/$OPEN_AI_KEY/g" src/api.rs
```

Run the build:

```bash
cargo build --release
```

Copy executable to .config:

```bash
cp target/release/gpt ~/.config
```

Add alias in your shell rc in this case .zshrc

```bash
echo 'alias gpt="~/.config/gpt \"\""' >> ~/.zshrc
```

Restart your shell and use:

```bash
gpt --help
```

# Usage

```bash
Usage: gpt [OPTIONS] [ARG]...

Arguments:
  [ARG]...

Options:
  -i, --info
      --config
      --color <COLOR>
      --code-color <COLOR>
      --max-tokens <0 - 16000>
      --temperature <0 - 2.0>
      --top-p <0 - 1.0>
      --frequency-penalty <0 - 2.0>
      --presence-penalty <0 - 2.0>
  -t, --translate <LANGUAGE>
  -c, --correct <LANGUAGE>
  -h, --help                         Print help
  -V, --version                      Print version
```

#### Exemple:

```bash
➜ ~ gpt -t german "Comment va tu ?"
Wie geht es dir?
```

```bash
➜  ~ gpt -c en "ho ar u"
How are you?
```

```bash
➜  ~ gpt --config --info -t en "Comment va tu ?"
CONFIG:
model: gpt-3.5-turbo-16k,
temperature: 1,
top_p: 1,
max_token: 200
frequency-penalty: 0
presence-penalty: 0

INFO:
model: gpt-3.5-turbo-16k-0613,
prompt: 18 tokens,
response: 5 tokens,
total: 23 tokens,
lifetime: 10811 tokens 0.0432 euro

How are you doing?

```

```bash
➜  ~ gpt Write me a shell command to search and replace A with B.

The 'sed' command can be used to search and replace text in a file. To replace A with B, you can use the following shell command:


sed -i 's/A/B/g' filename


Explanation:
- 'sed' is the command for stream editor.
- '-i' is used to edit the file in-place, i.e., modify the file directly.
- 's/A/B/g' is the substitute command in sed. It searches for A and replaces it with B. The 'g' flag is used to replace all occurrences within each line.
- 'filename' represents the name of the file in which you want to perform the search and replace operation. Replace 'filename' with the actual name of your file.

Make sure to replace 'A' and 'B' with the actual values you want to search and replace.

```
