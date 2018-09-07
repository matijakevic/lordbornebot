![Logo](https://i.nuuls.com/9BHHC.png)

A Twitch chat bot written in Rust

## Infrastructure
- lordbornebot_core crate - a library contains all types, macros, utilities and definitions that are shared between the bot and modules
- lordbornebot create - a binary that runs the actual IRC Client, loads and unloads the modules, initializes middleware etc. (basically glues everything together)

**IRC Client** (lordbornebot::twitch::client) ⇒ **decoding** (lordbornebot::twitch::parser) ⇒ **lordbornebot_core::Message** ⇒ **middleware** ⇒ **modules** ⇒ **encoding** (lordbornebot::twitch::parser) ⇒ **lordbornebot_core::Message** ⇒ **IRC Client Message Queue** (lordbornebot::twitch::client)

## Dynamic module system
1. Create a Cargo library project: `cargo new <module name> --lib`
2. Set the library type to cdylib by editing your Cargo.toml like this:
```
[package]
...

[lib]
crate-type = ["cdylib"]

[dependencies]
lordbornebot_core = {version = "*", git = "https://github.com/matijakevic/lordbornebot"}
...
```
3. In your lib.rs implement lordbornebot_core::Module trait for some type (struct for example).
4. Create an export function for your dynamic module (there might be a macro in the future to do this automatically):
```
#[no_mangle]
pub extern "C" fn _create_module(config: &Config) -> *mut Module {
   let obj = <construction of your object>;
   Box::into_raw(Box::new(obj))
}
```

5. Compile your module and copy the \<module name\> dynamic library into your modules directory.

See modules/ folder for examples.

### Pre-made modules
You still need to build them and copy the \<module name\> dynamic libraries into your modules directory!
- AFK module - utility module for notifying other chatters that some chatter is AFK.
- Points - a module for querying user's points.
- Gamble - a module for points gambling.
- Shape module - a module for receiving points on successfully created shape in chat.
- RPG module (WIP, stalled) - a large game module for MMORPGish dungeon experience.

## Middleware system
1. Implement middleware::Middleware trait for some type (struct for example).
2. Construct and add your type (Boxed) to middleware list inside init_middleware function in main.rs.

Middleware system is currently not dynamic. If it does become dynamic, it will use the same mechanism as dynamic modules.

### Pre-made middleware
- Filter - for filtering messages that may violate Twitch ToS / chat rules.

## How to run
1. Create a modules folder where you will put your module dynamic libraries.
2. Fill out example_config.json with your data.
   - command_prefx - a prefix that will be used to differentiate plain messages from commands (like "!", ">"...)
   - database_path - a path to the SQLite database, you can create one using migrations/create_tables.sql which contains all tables required for pre-made modules to work
   - banphrases_path - a path to json file containing a list of phrases which will indicate the filter system to ignore the messages containing those phrases, for example, this banphrases.json file
   ```
   ["abc"]
   ```
   will ignore all messages that contain word abc (in any case)
   - message_interval - the minimum time that needs to pass for IRC client to be able to send a message again (see Twitch IRC docs for rate limiting)
   - modules_path - a path to modules folder containing dynamic libraries of modules
   - modules - a list of dynamic modules that will be loaded automatically on startup
   - channels - a list of channels which will be joined automatically on startup
3. Path of the config.
   1. Set BOT_CONFIG_PATH environment variable to point to your config JSON file.
   2. Leaving it unset marks that configuration file is config.json in the directory where you run the bot.
4. `cargo run` to run the bot.
