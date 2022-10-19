# Twitch play rust

A simple bot to allow twitch chat to vote for the next input that will be played on stream.

The keyboard simulation part is done using the [Enigo crate](https://docs.rs/enigo/0.0.14/enigo/index.html) so it should work fine on Windows, MacOs, and Linux with X11 (unfortunately not in Wayland for the moment).


## Getting Started
### Configuration file

Add a `config.toml` file with the corresponding info :

- `name`: the nickname of the bot.
- `channel`: the twitch channel you want to join.
- `password`: the auth token you get by connecting to [https://twitchapps.com/tmi/](https://twitchapps.com/tmi/).
- `cooldown`: the number of seconds for the chat to vote.
- `commands`: a dictionnary of pairs (command name, key name) where you can choose command as you want to map to some Key on your keyboard. The name ofthe keys are the same as the Key enum [here](https://docs.rs/enigo/latest/enigo/enum.Key.html) (note: use "Meta" instead of Windows/Command/Super key since they are deprecated).

### Example of config file


```toml
name = "twitchplaybot"
channel = "yourchannelname"
password = "oauth:yourtokenfromtwitchapps"
cooldown = 30

[commands]
"l" = "LeftArrow"
"r" = "RighArrow"
"j" = "Space"
"p" = "Escape"
```

### Get the executable
#### From source
Install the rust toolchain and compile the project using `cargo build --release`. You may want to add the target flag, for instance with `--target x86_64-pc-windows-gnu` for WSL user that want to compile in order to run it directly in windows.

Then don't forget to add the `config.toml` file in the same directory as the executable file.

## Troubleshouting

If you experience some digits are pressed (or apparently nothing happened) instead of the arrow keys there is a quick fix : **disable your numlock**.
