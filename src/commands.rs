use crate::{fetch_database, parameters, storage};

use teloxide::{prelude::*, utils::command::BotCommand};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display info about bot.")]
    About,
    #[command(description = "update law database.")]
    Update,
    #[command(description = "get a law by id.")]
    Get,
}

pub async fn command_answer(
    cx: &UpdateWithCx<Message>,
    command: Command,
    parameters: std::sync::Arc<parameters::Parameters>,
    laws: std::sync::Arc<tokio::sync::Mutex<Box<dyn storage::LawStorageTrait + Send>>>,
) -> ResponseResult<()> {
    static ABOUT_TEXT: &str = "По всем замечаниям или предложениям обращаться сюда:\
        https://github.com/ZaMaZaN4iK/holywar-telegram . Спасибо!";

    match command {
        Command::About => {
            cx.reply_to(ABOUT_TEXT).send().await?;
        }
        Command::Update => {
            let new_laws =
                fetch_database::update_laws_database(&parameters.laws_database_uri).await;

            match new_laws {
                Ok(parsed_laws) => {
                    *laws.lock().await = parsed_laws;

                    log::info!(
                        "Laws database update executed successfully. Laws database size: {}",
                        laws.lock().await.len()
                    );
                }
                Err(e) => {
                    log::warn!(
                        "An error occurred during laws database update. The error: {}",
                        e
                    );
                }
            };
        }
        Command::Get => {}
    };

    Ok(())
}
