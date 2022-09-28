use enigo::{Enigo, Key, KeyboardControllable};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::time::SystemTime;

const URL: &str = "irc.twitch.tv:6667";
const CONFIG_FILE: &str = "config.toml";
// const TIMEOUT_SEC: u64 = 5;

lazy_static! {
    static ref CONFIG: Config = toml::from_str(
        &fs::read_to_string(CONFIG_FILE)
            .expect("config file not found you should create it (cf README)")
    )
    .unwrap();
    static ref PING_RE: Regex = Regex::new("PING :tmi.twitch.tv").unwrap();
    static ref COOLDOWN: u64 = CONFIG.cooldown;
    static ref CHANNEL: &'static String = &CONFIG.channel;
    static ref PASS: String = format!("PASS {}\n", &CONFIG.password);
    static ref NICK: String = format!("NICK {}\n", &CONFIG.name);
    static ref JOIN: String = format!("JOIN #{}\n", *CHANNEL);
    static ref PREFIX: &'static String = &CONFIG.prefix;
    static ref CMDS_LIST: String = CONFIG.commands.join("|");
    static ref CMD_PATTERN: String = format!("PRIVMSG #{} :{}({})", *CHANNEL, *PREFIX, *CMDS_LIST);
    static ref CMD_RE: Regex = Regex::new(&CMD_PATTERN).unwrap();
}

#[derive(Deserialize)]
struct Config {
    name: String,
    password: String,
    channel: String,
    cooldown: u64,
    prefix: String,
    commands: Vec<String>,
}

struct TwitchPlayBot {
    irc: TcpStream,
    reader: BufReader<TcpStream>,
}

impl TwitchPlayBot {
    fn send_msg(&mut self, msg: &String) -> std::io::Result<usize> {
        println!("DEBUG send_msg: {}", msg);
        self.irc.write(msg.as_bytes())
    }

    fn send_to_chat(&mut self, text: &String) {
        let raw_irc_msg = format!("PRIVMSG #{} :{}\r\n", *CHANNEL, text);
        self.send_msg(&raw_irc_msg)
            .expect("Send msg to chat failed");
    }

    fn auth(&mut self) {
        // https://dev.twitch.tv/docs/irc/authenticate-bot

        // 1) send a PASS message with oauth token to authenticate bot
        self.send_msg(&*PASS).expect("Auth failed with PASS msg");

        // 2) send a NICK message to give info about bot name
        self.send_msg(&*NICK).expect("Auth failed with NICK msg");

        // 3) send a JOIN message to make the bot joinning the chat
        self.send_msg(&*JOIN).expect("Auth failed with JOIN msg");
    }

    fn voted_command(&mut self, votes: &mut HashMap<String, usize>) -> Option<String> {
        // reset votes for every command to zero
        CONFIG.commands.iter().for_each(|k| {
            votes.entry(k.to_string()).and_modify(|c| *c = 0);
        });
        let start_t = SystemTime::now();

        // read lines until no
        let mut line = String::default();
        while self.reader.read_line(&mut line).unwrap() > 0 {
            println!("Bot received : {}", line);
            // answer to server PING msg to keep bot alive
            if PING_RE.is_match(line.as_str()) {
                self.send_msg(&"PONG :tmi.twitch.tv".to_owned()).unwrap();
            } else if let Some(captures) = CMD_RE.captures(&line) {
                // check if message is a command or not
                // get the cmd from the regex capture group CMD_PATTERN
                let cmd = captures.get(1).unwrap().as_str().to_owned();
                // update vote
                votes.entry(cmd).and_modify(|c| *c += 1);
            }
            // reset line cause read_line only append stuf
            line = String::default();
            if start_t.elapsed().unwrap().as_secs() >= *COOLDOWN {
                break;
            }
        }
        // select most voted

        let most_voted = votes.iter().max_by(|(_, c1), (_, c2)| c1.cmp(c2));
        match most_voted {
            None => {
                println!("Warning : Probably bad config THERE IS NO COMMANDS!");
                None
            }
            Some((_, 0)) => {
                println!("Zero vote has been received");
                None
            }
            Some((cmd, count_vote)) => {
                println!(
                    "Most voted command :  {}, Number of votes : {}",
                    cmd, count_vote
                );
                Some(cmd.to_string())
            }
        }
    }
    fn connect() -> TwitchPlayBot {
        let irc = TcpStream::connect(URL)
            .expect("Cannot connect twitch irc server check your internet connection :/");
        let reader = BufReader::new(irc.try_clone().unwrap());
        println!("Bot connected to {URL}");
        TwitchPlayBot { irc, reader }
    }
}

fn cmd_to_key(cmd: String) -> Key {
    let cmd = cmd.replace(*PREFIX, "");
    match cmd.as_str() {
        c if c.len() == 1 => Key::Layout(c.chars().next().unwrap()),
        "alt" => Key::Alt,
        "backspace" => Key::Backspace,
        "capslock" => Key::CapsLock,
        "control" => Key::Control,
        "delete" => Key::Delete,
        "down" => Key::DownArrow,
        "end" => Key::End,
        "esc" => Key::Escape,
        "f1" => Key::F1,
        "f2" => Key::F2,
        "f3" => Key::F3,
        "f4" => Key::F4,
        "f5" => Key::F5,
        "f6" => Key::F6,
        "f7" => Key::F7,
        "f8" => Key::F8,
        "f9" => Key::F9,
        "f10" => Key::F10,
        "f11" => Key::F11,
        "f12" => Key::F12,
        "home" => Key::Home,
        "left" => Key::LeftArrow,
        "meta" => Key::Meta,
        "option" => Key::Option,
        "pagedown" => Key::PageDown,
        "pageup" => Key::PageUp,
        "return" => Key::Return,
        "right" => Key::RightArrow,
        "shift" => Key::Shift,
        "space" => Key::Space,
        "tab" => Key::Tab,
        "up" => Key::UpArrow,
        _ => panic!("Unknown key"),
    }
}
fn main() -> std::io::Result<()> {
    assert_ne!(CONFIG.commands.len(), 0);
    let mut votes: HashMap<String, usize> =
        CONFIG.commands.iter().map(|cmd| (cmd.clone(), 0)).collect();
    let mut bot = TwitchPlayBot::connect();
    bot.auth();
    let mut enigo = Enigo::new();
    let greeting_msg = String::from("Bot is ready for taking chat commands!");
    let no_cmds = String::from("No commands received -> Bot did nothing :/");
    loop {
        bot.send_to_chat(&greeting_msg);
        let cmd = bot.voted_command(&mut votes);
        match cmd {
            None => bot.send_to_chat(&no_cmds),
            Some(cmd) => {
                bot.send_to_chat(&format!("Selected command : {cmd}"));
                enigo.key_click(cmd_to_key(cmd));
            }
        }
    }
}
