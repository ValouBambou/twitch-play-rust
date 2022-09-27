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
- `prefix`: the beginning of all commands to type in the chat.
- `commands`: the list of keys that the chat should control. For simple key just put the character and for special key add the name of the key. It should be the exact same names as Enigo Key enum you can check [here](https://docs.rs/enigo/latest/enigo/enum.Key.html) (note: use Meta instead of Windows/Command/Super key).

### Example of config file


```toml
name = "twitchplaybot"
channel = "yourchannelname"
password = "oauth:yourtokenfromtwitchapps"
cooldown = 30
prefix = "!"
commands = [
  "UpArrow",
  "DownArrow",
  "LeftArrow",
  "RightArrow",
  "Space",
  "Escape",
  "h",
  "j",
  "k",
  "l"
]
```

### Get the executable
#### From source
Install the rust toolchain and compile the project using `cargo build --release`. You may want to add the target flag, for instance with `--target x86_64-pc-windows-gnu` for WSL user that want to compile in order to run it directly in windows.

Then don't forget to add the `config.toml` file in the same directory as the executable file.
