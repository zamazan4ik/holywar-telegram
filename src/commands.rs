use crate::{entity, fetch_database, formatter, parameters, storage, utils};

use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(Clone, BotCommands)]
#[command(
    rename = "lowercase",
    description = "These commands are supported:",
    parse_with = "split"
)]
pub enum Command {
    #[command(description = "display info about bot.")]
    About,
    #[command(description = "update law database.")]
    Update,
    #[command(description = "get a law by id.")]
    Get(entity::LawNumber),
}

pub async fn command_handler(
    msg: Message,
    bot: AutoSend<Bot>,
    command: Command,
    parameters: std::sync::Arc<parameters::Parameters>,
    laws: std::sync::Arc<tokio::sync::Mutex<Box<dyn storage::LawStorageTrait + Send>>>,
) -> anyhow::Result<()> {
    static ABOUT_TEXT: &str = "По всем замечаниям или предложениям обращаться сюда:\
        https://github.com/ZaMaZaN4iK/holywar-telegram . Спасибо!";

    match command {
        Command::About => {
            bot.send_message(msg.chat.id, ABOUT_TEXT)
                .reply_to_message_id(msg.id)
                .await?;
        }
        Command::Update => {
            if utils::is_sender_an_owner(&msg.from(), parameters.owner_id) {
                let new_laws =
                    fetch_database::update_laws_database(&parameters.laws_database_uri).await;

                match new_laws {
                    Ok(parsed_laws) => {
                        *laws.lock().await = parsed_laws;

                        log::info!(
                            "Laws database update executed successfully. Laws database size: {}",
                            laws.lock().await.len()
                        );
                        bot.send_message(msg.chat.id, "Дело сделано!")
                            .reply_to_message_id(msg.id)
                            .await?;
                    }
                    Err(e) => {
                        log::warn!(
                            "An error occurred during laws database update. The error: {}",
                            e
                        );
                        bot.send_message(msg.chat.id, "Ржава не шмогла...")
                            .reply_to_message_id(msg.id)
                            .await?;
                    }
                };
            } else {
                bot.send_message(msg.chat.id, "Сынок, не лезь не в своё дело!")
                    .reply_to_message_id(msg.id)
                    .await?;
            }
        }
        Command::Get(law_number) => {
            let law_search_result = laws.lock().await.get_by_number(law_number).cloned();
            match law_search_result {
                Some(law) => {
                    bot.send_message(msg.chat.id, formatter::format_law(&law))
                        .reply_to_message_id(msg.id)
                        .await?;
                }
                None => {
                    bot.send_message(msg.chat.id, "Да нет такого закона!")
                        .reply_to_message_id(msg.id)
                        .await?;
                }
            }
        }
    };

    Ok(())
}
