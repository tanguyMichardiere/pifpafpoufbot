use argh::FromArgs;
use irc::client::prelude::*;
use std::collections::HashMap;
use std::thread;
use std::time;

#[derive(FromArgs)]
/// TODO
struct Args {
    /// config file
    #[argh(positional)]
    config: String,
    /// commands file
    #[argh(positional)]
    commands: Option<String>,
}

fn parse_config(path: &str) -> IrcClient {
    match Config::load(path) {
        Ok(config) => match IrcClient::from_config(config) {
            Ok(client) => client,
            Err(error) => panic!("error creating client from config file: {}", error),
        },
        Err(error) => panic!("error reading config file: {}", error),
    }
}

fn parse_commands(path: &str) -> HashMap<String, String> {
    match std::fs::read_to_string(path) {
        Ok(yaml) => match serde_yaml::from_str(&yaml) {
            Ok(commands) => commands,
            Err(error) => panic!("error parsing commands file: {}", error),
        },
        Err(error) => {
            panic!("error reading commands file: {}", error);
        }
    }
}

fn exponential_backoff(error: irc::error::IrcError, backoff: u32) {
    match backoff {
        0 => println!("error: {}\ntrying again immediately", error),
        _ => {
            let time = time::Duration::from_secs(2u64.pow(backoff - 1));
            println!("error: {}\ntrying again in {}s", error, time.as_secs());
            thread::sleep(time);
        }
    }
}

fn main() {
    let args: Args = argh::from_env();
    if !args.config.ends_with(".toml") {
        panic!("Expecting .toml config file");
    }
    let client = parse_config(&args.config);
    let commands;
    match args.commands {
        Some(name) => {
            if !name.ends_with(".yaml") {
                panic!("Expecting .yaml commands file");
            }
            commands = parse_commands(&name);
        }
        None => commands = HashMap::new(),
    }
    loop {
        for backoff in 0.. {
            match client.identify() {
                Ok(()) => break,
                Err(error) => exponential_backoff(error, backoff),
            }
        }
        match client.for_each_incoming(|irc_msg| {
            print!("{}", irc_msg);
            match match irc_msg.command {
                // respond to pings
                Command::PING(server1, server2) => Some(Command::PONG(server1, server2)),
                // static commands
                Command::PRIVMSG(msgtarget, message) => match commands.get(&message) {
                    Some(command_response) => {
                        Some(Command::PRIVMSG(msgtarget, command_response.to_string()))
                    }
                    None => None,
                },
                _ => None,
            } {
                Some(command) => {
                    println!("sending command {:?}", command);
                    match client.send(command.clone()) {
                        Ok(()) => (),
                        Err(error) => {
                            println!(
                                "error: {}\nstarting exponential backoff routine asynchronously",
                                error
                            );
                            let client = client.clone();
                            thread::spawn(move || {
                                for backoff in 1.. {
                                    match client.send(command.clone()) {
                                        Ok(()) => break,
                                        Err(error) => exponential_backoff(error, backoff),
                                    }
                                }
                            });
                        }
                    }
                }
                None => (),
            }
        }) {
            Ok(()) => println!("this should never happen"),
            Err(error) => println!("error: {}\nreconnecting", error),
        }
    }
}
