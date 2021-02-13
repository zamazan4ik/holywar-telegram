pub struct Parameters {
    pub bot_name: String,
    pub owner_id: i32,
    pub laws_database_uri: String,
    pub database_update_periodicity: std::time::Duration,
    pub is_webhook_mode_enabled: bool,
}

impl Parameters {
    pub fn new() -> Self {
        let bot_name = std::env::var("BOT_NAME").expect("BOT_NAME env var is not specified");

        let owner_id: i32 = std::env::var("OWNER_ID")
            .expect("OWNER_ID env var is not specified")
            .parse()
            .expect("Cannot parse as i32");

        let laws_database_uri = std::env::var("LAWS_DATABASE_URI").unwrap_or(
            "https://raw.githubusercontent.com/ZaMaZaN4iK/holywar_laws/main/holywar_rules.yaml"
                .to_string(),
        );

        let database_update_periodicity = std::time::Duration::from_secs(
            std::env::var("DATABASE_UPDATE_PERIODICITY_IN_SECONDS")
                .unwrap_or(
                    std::time::Duration::from_secs(24 * 60 * 60)
                        .as_secs()
                        .to_string(),
                )
                .parse()
                .expect("Cannot parse provided time as seconds"),
        );

        let is_webhook_mode_enabled: bool = std::env::var("WEBHOOK_MODE")
            .unwrap_or("false".to_string())
            .parse()
            .expect(
                "Cannot convert WEBHOOK_MODE to bool. Applicable values are only \"true\" or \"false\"",
            );

        Self {
            bot_name,
            owner_id,
            laws_database_uri,
            database_update_periodicity,
            is_webhook_mode_enabled,
        }
    }
}
