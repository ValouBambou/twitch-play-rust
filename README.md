# Twitch play rust

A simple bot to allow twitch chat to vote for the next input that will be played on stream.

The keyboard simulation part is done using the [TFC crate](https://docs.rs/tfc/latest/tfc/) so it should work fine on Windows, MacOs, and Linux (for Wayland extra steps are needed according to the [README](https://github.com/indianakernick/The-Fat-Controller)).


## Getting Started
### Configuration file

Add a `config.toml` file with the corresponding info :

- `name`: the nickname of the bot.
- `channel`: the twitch channel you want to join.
- `password`: the auth token you get by connecting to [https://twitchapps.com/tmi/](https://twitchapps.com/tmi/).
- `cooldown`: the number of seconds for the chat to vote.
- `commands`: a dictionnary of pairs (command name, key name) where you can choose command as you want to map to some Key on your keyboard. The name of the keys are the same as the Key enum [here](https://docs.rs/tfc/latest/tfc/enum.Key.html).

### Example of config file


```toml
name = "twitchplaybot"
channel = "yourchannelname"
password = "oauth:yourtokenfromtwitchapps"
cooldown = 30

[commands]
"left" = "LeftArrow"
"right" = "RightArrow"
"jump" = "Space"
"pause" = "Escape"
```

### Get the executable
#### From source
Install the rust toolchain and compile the project using `cargo build --release`. You may want to add the target flag, for instance with `--target x86_64-pc-windows-gnu` for WSL user that want to compile in order to run it directly in windows.

Then don't forget to add the `config.toml` file in the same directory as the executable file.
