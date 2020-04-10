use irc::client::prelude::*;

fn main() {
    let yaml = std::fs::read_to_string("commands.yaml").expect("Error reading commands.yaml");
    let commands: std::collections::HashMap<String, String> =
        serde_yaml::from_str(&yaml).expect("Error parsing commands.yaml");
    let client = IrcClient::new("config.toml").expect("Error parsing config.toml");
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
