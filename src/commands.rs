use crate::{entity, fetch_database, formatter, parameters, storage, utils};

use teloxide::{prelude::*, utils::command::BotCommand};

#[derive(BotCommand)]
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
            if utils::is_sender_an_owner(&cx.update.from(), parameters.owner_id) {
                let new_laws =
                    fetch_database::update_laws_database(&parameters.laws_database_uri).await;

                match new_laws {
                    Ok(parsed_laws) => {
                        *laws.lock().await = parsed_laws;

                        log::info!(
                            "Laws database update executed successfully. Laws database size: {}",
                            laws.lock().await.len()
                        );
                        cx.reply_to("Дело сделано!").send().await?;
                    }
                    Err(e) => {
                        log::warn!(
                            "An error occurred during laws database update. The error: {}",
                            e
                        );
                        cx.reply_to("Ржава не шмогла...").send().await?;
                    }
                };
            } else {
                cx.reply_to("Сынок, не лезь не в своё дело!").send().await?;
            }
        }
        Command::Get(law_number) => {
            let law_search_result = laws.lock().await.get_by_number(law_number).cloned();
            match law_search_result {
                Some(law) => {
                    cx.reply_to(formatter::format_law(&law)).send().await?;
                }
                None => {
                    cx.reply_to("Да нет такого закона!").send().await?;
                }
            }
        }
    };

    Ok(())
}
