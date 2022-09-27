# Twitch play rust

A simple bot to allow twitch chat to vote for the next input that will be played on stream.

## Requirements
A config.toml file with the correspondig info:

- `name`: the nickname of the bot.
- `channel`: the twitch channel you want to join.
- `password`: the auth token you get by connecting to [https://twitchapps.com/tmi/](https://twitchapps.com/tmi/).
- `cooldown`: the number of seconds for the chat to vote.

## TODO
- parse messages
- voting system
- search for how to emulate a key pressed
- error handling (some auto restart or more advanced stuff)
