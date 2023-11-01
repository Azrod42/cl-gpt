#!/bin/bash
#
#sed -i "s/YOUR_API_KEY/$OPEN_AI_KEY/g" src/api.rs
perl -pi -e "s/YOUR_API_KEY/$OPEN_AI_KEY/g" src/api.rs
cargo build --release
cp target/release/gpt ~/.config
echo 'alias gpt="~/.config/gpt \"\""' >> ~/.zshrc
