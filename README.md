# PifpafpoufBot

A very simple Twitch chat bot written in Rust

- [PifpafpoufBot](#pifpafpoufbot)
  - [Roadmap](#roadmap)
  - [Installation](#installation)
  - [Configuration](#configuration)
    - [IRC configuration](#irc-configuration)
    - [Commands configuration](#commands-configuration)
      - [Static commands](#static-commands)

## Roadmap

- [x] static commands
- [ ] custom command prefix
- [ ] dynamic commands
  - [ ] uptime
  - [ ] add a static command (temporarily or permanently)

## Installation

```
git clone git@github.com:tanguyMichardiere/pifpafpoufbot.git
cd pifpafpoufbot
cargo run --release
```

## Configuration

### IRC configuration

Minimal config:

```toml
# config.toml

server = "irc.chat.twitch.tv"
port = 6697
use_ssl = true
channels = ["#<your-twitch-channel-in-lowercase>"]
password = "oauth:<your-oauth-token"
nickname = "<the-bot's-twitch-name>"
```

See [this Twitch Developpers guide](https://dev.twitch.tv/docs/irc/guide#connecting-to-twitch-irc) to generate your oauth token.

See [the iirc crate on GitHub](https://github.com/aatxe/irc) for an exhaustive list of configuration fields.

### Commands configuration

#### Static commands

Commands of which the response is always the same are stored in a HashMap read from a yaml file. The bot listens to all messages on its channel(s) and if a message is equal to a key, the value associated is sent in the same channel.

Example commands:

```yaml
# commands.yaml

---
"!ping": pong
"!uptime": no idea i'm a static response
```