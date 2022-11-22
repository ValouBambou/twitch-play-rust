use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
use tfc::{traits::KeyboardContext, Context, Enum, Key};

const URL: &str = "irc.twitch.tv:6667";
const CONFIG_FILE: &str = "config.toml";

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
    static ref CMDS_RE: String = CONFIG
        .commands
        .keys()
        .fold(String::new(), |a, b| a + b + "|");
    static ref KEY_FROM_CMD: HashMap<String, Key> = CONFIG
        .commands
        .iter()
        .map(|(cmd, key)| (cmd.to_string(), key_from_string(key.to_string())))
        .collect();
    static ref CMD_PATTERN: String = format!("PRIVMSG #{} :({})", *CHANNEL, *CMDS_RE);
    static ref CMD_RE: Regex = Regex::new(&CMD_PATTERN).unwrap();
}

fn key_from_string(cmd: String) -> Key {
    Key::iter().find(|k| k.identifier_name() == cmd).expect("Key doesn't exist, invalid config should be exact match with Enum Variant name from TFC library.")
}

#[derive(Deserialize)]
struct Config {
    name: String,
    password: String,
    channel: String,
    cooldown: u64,
    commands: HashMap<String, String>,
}

struct TwitchPlayBot {
    irc: TcpStream,
    votes: HashMap<String, usize>,
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

    fn voted_command(&mut self) -> Option<String> {
        // reset votes for every command to zero
        for (_, vote) in self.votes.iter_mut() {
            *vote = 0;
        }
        // sleep to let chat vote
        // we sleep less to anticipate the delay of restauring the connection
        assert!(*COOLDOWN > 0);
        sleep(Duration::from_millis(1000 * (*COOLDOWN) - 200));

        self.irc.shutdown(std::net::Shutdown::Both).unwrap();
        let mut irc_msgs = String::default();
        self.irc.read_to_string(&mut irc_msgs).unwrap();

        for line in irc_msgs.split("\r\n") {
            println!("Bot received: {}", line);
            // answer to server PING msg to keep bot alive
            if PING_RE.is_match(line) {
                self.send_msg(&"PONG :tmi.twitch.tv".to_owned()).unwrap();
            } else if let Some(captures) = CMD_RE.captures(&line) {
                // check if message is a command or not
                // get the cmd from the regex capture group CMD_PATTERN
                let cmd = captures.get(1).unwrap().as_str().to_owned();
                // update vote
                self.votes.entry(cmd).and_modify(|c| *c += 1);
            }
        }

        // re connect and authenticate
        self.irc = TcpStream::connect(URL).expect("Connection failed ;/");
        self.auth();

        // select most voted
        let most_voted = self.votes.iter().max_by(|(_, c1), (_, c2)| c1.cmp(c2));

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

    fn start(&mut self) {
        let mut ctx = Context::new().unwrap();
        self.send_to_chat(&"Bot connected ready to received commands !".to_owned());
        loop {
            let opt_cmd = self.voted_command();
            let msg = match opt_cmd {
                Some(cmd) => {
                    let key = KEY_FROM_CMD.get(&cmd).unwrap().clone();
                    ctx.key_down(key.clone()).unwrap();
                    sleep(Duration::from_millis(10));
                    ctx.key_up(key).unwrap();
                    format!("Most voted command : {}", cmd)
                }
                None => "No command received, did nothing :/".to_owned(),
            };
            self.send_to_chat(&msg);
        }
    }

    fn connect() -> TwitchPlayBot {
        let votes: HashMap<String, usize> = CONFIG
            .commands
            .iter()
            .map(|(cmd, _)| (cmd.clone(), 0))
            .collect();
        let irc = TcpStream::connect(URL)
            .expect("Cannot connect twitch irc server check your internet connection :/");
        println!("Bot connected to {URL}");
        TwitchPlayBot { irc, votes }
    }
}

fn main() {
    assert_ne!(CONFIG.commands.len(), 0);
    let mut bot = TwitchPlayBot::connect();
    bot.auth();
    bot.start();
}
