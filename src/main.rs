mod commands;
mod entity;
mod fetch_database;
mod formatter;
mod logging;
mod parameters;
mod storage;
mod utils;
mod webhook;

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    logging::init_logger();
    log::info!("Starting Holywar bot");

    if let Ok(dotenv_absolute_path) = std::env::var("DOTENV_ABSOLUTE_PATH") {
        match dotenv::from_path(std::path::PathBuf::from(dotenv_absolute_path.clone())) {
            Ok(_) => log::info!(
                "Environment variables are loaded successfully from .env file. .env filepath: {}",
                dotenv_absolute_path
            ),
            Err(e) => log::info!(
                "Environment variables are not loaded from .env file. Error: {}",
                e
            ),
        };
    }

    let parameters = std::sync::Arc::new(parameters::Parameters::new());

    let bot = Bot::from_env().auto_send();

    let laws: std::sync::Arc<tokio::sync::Mutex<Box<dyn storage::LawStorageTrait + Send>>> =
        std::sync::Arc::new(tokio::sync::Mutex::new(Box::new(
            storage::LawStorage::new_empty(),
        )));

    tokio::spawn(fetch_database::update_laws_database_periodic_task(
        laws.clone(),
        parameters.laws_database_uri.clone(),
        parameters.database_update_periodicity.clone(),
    ));

    let handler = Update::filter_message().branch(
        dptree::entry()
            .filter_command::<commands::Command>()
            .endpoint(commands::command_handler),
    );

    if !parameters.is_webhook_mode_enabled {
        log::info!("Webhook deleted");
        bot.delete_webhook().await.expect("Cannot delete a webhook");
    }

    let mut bot_dispatcher = Dispatcher::builder(bot.clone(), handler)
        .dependencies(dptree::deps![parameters.clone(), laws.clone()])
        .default_handler(|_| async move {})
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build();

    if parameters.is_webhook_mode_enabled {
        log::info!("Webhook mode activated");
        let rx = webhook::webhook(bot);
        bot_dispatcher
            .dispatch_with_listener(
                rx.await,
                LoggingErrorHandler::with_custom_text("An error from the update listener"),
            )
            .await;
    } else {
        log::info!("Long polling mode activated");
        bot_dispatcher.dispatch().await;
    }
}
