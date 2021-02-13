use crate::storage;

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
    /*.json::<std::collections::HashMap<String, storage::Law>>()
    .await?;*/

    let new_laws = storage::LawStorage::new_empty();
    for document in serde_yaml::Deserializer::from_str(resp.text().await?.as_str()) {
        let value = serde_yaml::Value::deserialize(document)?;
        println!("{:?}", value);
    }

    let new_laws = Box::new(new_laws);

    Ok(new_laws)
}
