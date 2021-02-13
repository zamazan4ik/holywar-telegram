mod commands;
mod fetch_database;
mod logging;
mod parameters;
mod storage;
mod webhook;

use teloxide::{prelude::*, utils::command::BotCommand};

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    logging::init_logger();
    log::info!("Starting Holywar bot");

    let use_dotenv: bool = std::env::var("USE_DOTENV")
        .unwrap_or("false".to_string())
        .parse()
        .expect(
            "Cannot convert USE_DOTENV to bool. Applicable values are only \"true\" or \"false\"",
        );

    if use_dotenv {
        match dotenv::dotenv() {
            Ok(path) => log::info!(
                "Environment variables are loaded successfully from .env file. .env filepath: {:?}",
                path
            ),
            Err(e) => log::info!(
                "Environment variables are not loaded from .env file. Error: {:?}",
                e
            ),
        };
    }

    let parameters = std::sync::Arc::new(parameters::Parameters::new());
    let webhook_mode = parameters.is_webhook_mode_enabled;

    let bot = Bot::from_env();

    let laws: std::sync::Arc<tokio::sync::Mutex<Box<dyn storage::LawStorageTrait + Send>>> =
        std::sync::Arc::new(tokio::sync::Mutex::new(Box::new(
            storage::LawStorage::new_empty(),
        )));

    tokio::spawn(fetch_database::update_laws_database_periodic_task(
        laws.clone(),
        parameters.laws_database_uri.clone(),
        parameters.database_update_periodicity.clone(),
    ));

    let bot_dispatcher =
        Dispatcher::new(bot.clone()).messages_handler(move |rx: DispatcherHandlerRx<Message>| {
            rx.for_each(move |message| {
                let laws = laws.clone();
                let parameters = parameters.clone();

                async move {
                    let message_text = match message.update.text() {
                        Some(x) => x,
                        None => return,
                    };

                    // Handle commands. If command cannot be parsed - continue processing
                    match commands::Command::parse(message_text, &parameters.bot_name) {
                        Ok(command) => {
                            commands::command_answer(
                                &message,
                                command,
                                parameters.clone(),
                                laws.clone(),
                            )
                            .await
                            .log_on_error()
                            .await;
                            ()
                        }
                        Err(_) => (),
                    };
                }
            })
        });

    if webhook_mode {
        log::info!("Webhook mode activated");
        let rx = webhook::webhook(bot);
        bot_dispatcher
            .dispatch_with_listener(
                rx.await,
                LoggingErrorHandler::with_custom_text("An error from the update listener"),
            )
            .await;
        return;
    }

    log::info!("Long polling mode activated");
    bot.delete_webhook()
        .send()
        .await
        .expect("Cannot delete a webhook");
    bot_dispatcher.dispatch().await;
}
