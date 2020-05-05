# PifpafpoufBot

A very simple Twitch chat bot written in Rust

- [PifpafpoufBot](#pifpafpoufbot)
  - [Roadmap](#roadmap)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Included commands](#included-commands)
  - [Configuration](#configuration)
    - [Main configuration file](#main-configuration-file)
      - [Additional options](#additional-options)
    - [Static commands configuration](#static-commands-configuration)

## Roadmap

- [x] static commands
- [x] custom command prefix
- [ ] timeout between commands
- [ ] periodic message
- [ ] dynamic commands
  - [ ] uptime
  - [ ] help
  - [ ] add a static command (temporarily or permanently)

## Installation

build dependency: [OpenSSL](https://docs.rs/openssl/0.10.16/openssl/#automatic)

```Shell
cargo install --git https://github.com/tanguyMichardiere/pifpafpoufbot.git pifpafpoufbot
```

## Usage

```Shell
pifpafpoufbot config.toml [commands.toml]
```

### Included commands

* uptime: get the current uptime of the stream
* help: get a list of available commands
* add: add a temporary static command
* add!: add a permanent static command (editing ```commands.toml```)

## Configuration

### Main configuration file

Minimal config:

```toml
# config.toml

server = "irc.chat.twitch.tv"
port = 6697
use_ssl = true
channels = ["#<your-twitch-channel-in-lowercase>"]
password = "oauth:<your-oauth-token"
nickname = "<the-bot's-twitch-name-in-lowercase>"
```

See [this Twitch Developpers guide](https://dev.twitch.tv/docs/irc/guide#connecting-to-twitch-irc) to generate your oauth token.

See [the irc crate on GitHub](https://github.com/aatxe/irc) for an exhaustive list of configuration fields.

#### Additional options

In case you want to modify what the bot recognizes as commands, you can add an ```[options]``` section at the end of ```config.toml``` with the following (all optional) fields:

```toml
#config.toml

...

[options]
prefix = "!"   # prefix to all commands
uptime = "uptime"
help = "help"
add = "add"
add_permanent = "add!"
```

The values given here are the default ones.

### Static commands configuration

Commands of which the response is always the same are stored in a HashMap read from a toml file. The bot listens to all messages on its channel and if a message is equal to a key (with the prefix configured in the main configuration file), the value associated is sent in the same channel.

Example commands:

```toml
# commands.toml

ping = "pong"
planning = "www.yourwebsite.com/planning"
```
