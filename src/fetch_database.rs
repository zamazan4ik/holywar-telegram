use crate::{entity, storage};

use crate::storage::LawStorageTrait;
use serde::Deserialize;

pub async fn update_laws_database_periodic_task(
    laws: std::sync::Arc<tokio::sync::Mutex<Box<dyn storage::LawStorageTrait + Send>>>,
    uri: String,
    update_periodicity: std::time::Duration,
) {
    loop {
        let new_laws = update_laws_database(&uri).await;

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
        }

        tokio::time::delay_for(update_periodicity).await;
    }
}

pub async fn update_laws_database(
    uri: &String,
) -> anyhow::Result<Box<dyn storage::LawStorageTrait + Send>> {
    let resp = reqwest::get(uri).await?;

    let mut new_laws = storage::LawStorage::new_empty();
    for document in serde_yaml::Deserializer::from_str(resp.text().await?.as_str()) {
        let value = serde_yaml::Value::deserialize(document)?;

        if let Some(law_sequence) = value.as_sequence() {
            for law_mapping in law_sequence {
                let parsed_law = serde_yaml::from_value::<entity::Law>(law_mapping.clone());

                match parsed_law {
                    Ok(law) => {
                        log::debug!("Parsed law: {:?}", law);
                        new_laws.add(law);
                    }
                    Err(e) => log::warn!("Cannot parse a law: {}", e),
                };
            }
        } else {
            log::warn!("Cannot parse law sequence")
        }
    }

    let new_laws = Box::new(new_laws);

    Ok(new_laws)
}
