![Logo](https://i.nuuls.com/9BHHC.png)

A Twitch chat bot written in Rust

## How to run
1. Fill out example_config.json with your data.
2. Path of the config.
   1. Set BOT_CONFIG_PATH environment variable to point to your config JSON file.
   2. Leaving it unset marks that config.json file is in the directory where you run the bot.
3. `cargo run` to run the bot.

## Module system
1. Implement modules::Module trait for some type (struct for example).
2. Construct and add your type (Boxed) using init_modules functions in main.rs.