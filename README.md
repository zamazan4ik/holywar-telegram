# holywar-telegram
[![GitHub code size](https://img.shields.io/github/languages/code-size/ZaMaZaN4iK/holywar-telegram?style=flat)](https://github.com/ZaMaZaN4iK/holywar-telegram)
### About
Just a bot for the best Telegram channel.

### Dependencies
* [Rust](https://www.rust-lang.org/) 1.64 or newer
* Cargo

Possibly will work with other Rust versions, but I didn't test it.

### How to build
* Clone this repository
* `cargo build --release`

### How to run
I recommend to run this bot as a service (e.g. as systemd service) on a machine.
Also Docker images are available here: https://hub.docker.com/repository/docker/zamazan4ik/holywar-telegram

### Configuration
The bot can be configured only with environment variables. For now there are we support the following variables:

| Name | Description | Values | Default value | Required |
|------|-------------|--------|---------------|----------|
| TELOXIDE_TOKEN | Telegram bot token | Any valid and registered Telegram bot token | None | All mods |
| WEBHOOK_MODE | Run bot in webhook mode or long-polling mode | `true` for webhook, 'false' for long-polling | `false` | All mods |
| BIND_ADDRESS | Address for binding the web-service | Any valid IP address | `0.0.0.0` | Webhook mode |  
| BIND_PORT | Port for binding the web-service | Any valid port | `8080` | Webhook mode |
| HOST | Host, where Telegram will send updates in webhook mode | Any valid host address | None | Webhook mode |
| WEBHOOK_URI | An URI path for webhook mode like **/api/v1/some_secret_endpoint** | Any valid URI path | TELOXIDE_TOKEN | Webhook mode |
| LAWS_DATABASE_URI | An URI path with law database | Any valid URI path | https://raw.githubusercontent.com/ZaMaZaN4iK/holywar_laws/main/holywar_rules.yaml | All mods |
| DATABASE_UPDATE_PERIODICITY_IN_SECONDS | Law database update periodicity in seconds | Any positive int | 86400 | All mods |
| DOTENV_ABSOLUTE_PATH | For development purposes only. Filepath to .env file | Any valid filepath | None (but it's not required) | All mods |

If for any variable there is no default value and you didn't provide any value - the bot won't start (the only exception is `DOTENV_ABSOLUTE_PATH` env var).
Bot automatically registers webhook (if is launched in webhook mode) with address `https://$HOST/$TELOXIDE_TOKEN`, if no `WEBHOOK_URI` is provided.

### How to use
Just add the bot to your chat and use it's commands.

### Feedback
If you have any suggestions or want to report a bug - feel free to create an issue in this repo. Thank you!
