use chrono::{DateTime, Utc};
use mongodb::{bson::doc, error::Result, options::ClientOptions, Client, Collection, Database};

use crate::fragments::ConfirmationInfoForm;

#[derive(Clone)]
pub struct ParticipantsRepository {
    confirmations: Collection<ConfirmationInfo>,
}

impl ParticipantsRepository {
    pub fn new(database: &Database) -> Self {
        Self {
            confirmations: database.collection("Participants"),
        }
    }

    pub async fn upsert(&self, mut form: ConfirmationInfoForm) -> Result<()> {
        form.escorts.retain(|e| !e.is_empty());

        let escorts = if form.escorts.is_empty() {
            None
        } else {
            Some(form.escorts.iter().map(|e| e.trim().to_owned()).collect())
        };

        let conf = ConfirmationInfo {
            time: chrono::Utc::now(),
            name: form.name.trim().to_owned(),
            escorts,
        };

        let filter = doc! {"name": &conf.name};

        let exists = self
            .confirmations
            .find_one(filter.clone(), None)
            .await?
            .is_some();

        if exists {
            let update = doc! { "$set": { "escorts": conf.escorts } };
            self.confirmations.update_one(filter, update, None).await?;
        } else {
            self.confirmations.insert_one(conf, None).await?;
        }

        Ok(())
    }

    pub async fn get_info(&self, name: &str) -> Result<Option<ConfirmationInfo>> {
        let transactions = self
            .confirmations
            .find_one(doc! { "name": name }, None)
            .await?;

        Ok(transactions)
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ConfirmationInfo {
    pub time: DateTime<Utc>,
    pub name: String,
    pub escorts: Option<Vec<String>>,
}

pub async fn get_mongo_client() -> Result<Client> {
    dotenv::dotenv().ok();

    let connection_string = std::env::var("MONGO_CONNECTION_STRING")
        .unwrap_or_else(|_| "mongodb://localhost/?retryWrites=true".to_string());

    let options = ClientOptions::parse(connection_string).await?;

    Client::with_options(options)
}
