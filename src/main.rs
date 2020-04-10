use irc::client::prelude::*;
use std::collections::HashMap;

fn main() {
    let commands: HashMap<String, String>;
    match std::fs::read_to_string("commands.yaml") {
        Ok(yaml) => commands = serde_yaml::from_str(&yaml).expect("Error parsing commands.yaml"),
        Err(_e) => {
            println!("commands.yaml not found, bot starting with no static command");
            commands = HashMap::new()
        }
    }
    let client: IrcClient;
    match Config::load("config.toml") {
        Ok(config) => {
            client = IrcClient::from_config(config).expect("Error creating client from config.toml")
        }
        Err(e) => panic!("Error reading config.toml: {}", e),
    }
    client.identify().expect("Error identifying");
    client
        .for_each_incoming(|irc_msg| {
            print!("{}", irc_msg);
            let mut response: Option<Command> = None;
            match irc_msg.command {
                Command::PING(server1, server2) => response = Some(Command::PONG(server1, server2)),
                Command::PRIVMSG(msgtarget, message) => match commands.get(&message) {
                    Some(command_response) => {
                        response = Some(Command::PRIVMSG(msgtarget, command_response.to_string()))
                    }
                    None => (),
                },
                _ => (),
            }
            match response {
                Some(command) => {
                    println!("sending command {:?}", command);
                    client.send(command).expect("Error sending command");
                }
                None => (),
            }
        })
        .unwrap();
}
