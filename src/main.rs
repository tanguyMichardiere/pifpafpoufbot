use argh::FromArgs;
use irc::client::prelude::*;
use std::collections::HashMap;
use std::thread;
use std::time;

#[derive(FromArgs)]
/// A customizable Twitch chat bot
struct Args {
    /// config file
    #[argh(positional)]
    config: String,
    /// commands file
    #[argh(positional)]
    commands: Option<String>,
}

/// parse a toml config file and create an IrcClient from it
fn parse_config(path: &str) -> IrcClient {
    IrcClient::from_config(Config::load(path).unwrap()).unwrap()
}

/// parse a toml file and create a HashMap from it
/// will panic if the toml file contains values of an other type than String
fn parse_commands(path: &str) -> HashMap<String, String> {
    toml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

/// perform exponential backoff, typically used in a for loop:
/// ```
/// for backoff in 0.. {
///     match some_network_function_that_can_fail() {
///         Ok(_) => break,
///         Err(error) => exponential_backoff(error, backoff),
///     }
/// }
/// ```
fn exponential_backoff(error: irc::error::IrcError, backoff: u32) {
    match backoff {
        0 => {
            eprintln!("{}", error);
            eprintln!("trying again immediately")
        }
        _ => {
            let time = time::Duration::from_secs(2u64.pow(backoff - 1));
            eprintln!("{}", error);
            eprintln!("trying again in {}s", time.as_secs());
            thread::sleep(time);
        }
    }
}

fn uptime() -> String {
    String::from("TODO: uptime")
}

fn help() -> String {
    String::from("TODO: help")
}

fn add(_command: &str) -> String {
    String::from("TODO: add")
}

fn add_permanent(_command: &str) -> String {
    String::from("TODO: add permanent")
}

fn main() {
    let args: Args = argh::from_env();
    if !args.config.ends_with(".toml") {
        panic!("Expecting .toml config file");
    }
    let client = parse_config(&args.config);
    let commands = match args.commands {
        Some(name) => {
            if !name.ends_with(".toml") {
                panic!("Expecting .toml commands file");
            }
            parse_commands(&name)
        }
        None => {
            println!("No commands file provided");
            HashMap::new()
        }
    };
    let prefix = client.config().get_option("prefix").unwrap_or("!");
    let uptime_command = client.config().get_option("uptime").unwrap_or("uptime");
    let help_command = client.config().get_option("help").unwrap_or("help");
    let add_command = client.config().get_option("add").unwrap_or("add");
    let add_permanent_command = client
        .config()
        .get_option("add_permanent")
        .unwrap_or("add!");
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
                // commands
                Command::PRIVMSG(msgtarget, message) => {
                    if message.starts_with(prefix) {
                        let command = &message[prefix.len()..];
                        match commands.get(command) {
                            // static commands
                            Some(response) => {
                                Some(Command::PRIVMSG(msgtarget, response.to_string()))
                            }
                            None => {
                                // dynamic commands
                                if command == uptime_command {
                                    Some(Command::PRIVMSG(msgtarget, uptime()))
                                } else if command == help_command {
                                    Some(Command::PRIVMSG(msgtarget, help()))
                                } else {
                                    // dynamic commands with arguments
                                    if command.starts_with(add_command) {
                                        Some(Command::PRIVMSG(msgtarget, add(command)))
                                    } else if command.starts_with(add_permanent_command) {
                                        Some(Command::PRIVMSG(msgtarget, add_permanent(command)))
                                    } else {
                                        None
                                    }
                                }
                            }
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            } {
                Some(command) => {
                    println!("sending command {:?}", command);
                    match client.send(command.clone()) {
                        Ok(()) => (),
                        Err(error) => {
                            eprintln!("{}", error);
                            eprintln!("starting exponential backoff routine asynchronously");
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
            Ok(()) => eprintln!("this should never happen"),
            Err(error) => {
                eprintln!("{}", error);
                eprintln!("reconnecting")
            }
        }
    }
}
