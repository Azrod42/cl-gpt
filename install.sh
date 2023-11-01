#!/bin/bash
#
sed -i "s/YOUR_API_KEY/$OPEN_AI_KEY/g" src/api.rs
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo build --release
cp target/release/gpt ~/.config
echo 'alias gpt="~/.config/gpt \"\""' >> ~/.zshrc
