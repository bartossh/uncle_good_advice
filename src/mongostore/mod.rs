use crate::{shared::SentimentData, traits::Store};
use derive_builder::Builder;
use kalosm::language::StreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId},
    Client, Database,
};
use std::fmt::Debug;

const COLLECTION_NAME: &str = "sentiment_reports";

#[derive(Debug, Builder)]
pub struct Uri {
    url_with_credentials: String,
    database: String,
    tls: bool,
}

impl Into<String> for Uri {
    fn into(self) -> String {
        format!(
            "mongodb://{}/{}?tls={}",
            self.url_with_credentials, self.database, self.tls,
        )
    }
}

#[derive(Debug)]
pub struct Storage {
    db: Database,
}

impl Storage {
    pub async fn try_new(uri: impl Into<String>, db_name: &str) -> mongodb::error::Result<Self> {
        let client = Client::with_uri_str(uri.into()).await?;

        Ok(Self {
            db: client.database(db_name),
        })
    }
}

impl<'a> Store<'a, SentimentData> for Storage {
    async fn save(&self, entity: &SentimentData) -> Result<String, String> {
        let result = self
            .db
            .collection::<SentimentData>(COLLECTION_NAME)
            .insert_one(entity)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result
            .inserted_id
            .as_object_id()
            .unwrap_or_default()
            .to_hex())
    }

    async fn read_by_id(&self, id: &str) -> Result<SentimentData, String> {
        let obj_id = ObjectId::parse_str(id).map_err(|e| e.to_string())?;
        let result = self
            .db
            .collection::<SentimentData>(COLLECTION_NAME)
            .find_one(doc! { "_id": obj_id })
            .await
            .map_err(|e| e.to_string())?
            .unwrap_or_default();
        Ok(result)
    }

    async fn read_from_time(&self, timestamp_ms: u128) -> Result<Vec<SentimentData>, String> {
        let query = doc! { "created_at": { "$gte": bson::Decimal128::from_bytes(timestamp_ms.to_be_bytes())}};

        let mut cursor = self
            .db
            .collection::<SentimentData>(COLLECTION_NAME)
            .find(query)
            .await
            .map_err(|e| e.to_string())?;

        let mut result = Vec::new();
        while let Some(d) = cursor.next().await {
            if let Ok(entity) = d {
                result.push(entity);
            }
        }

        Ok(result)
    }
}

#[cfg(feature = "integrations")]
mod tests {

    #[tokio::test]
    async fn it_should_connect_to_the_store() {
        use super::{Storage, Uri, UriBuilder};

        let uri: Uri = UriBuilder::default()
            .url_with_credentials("localhost:27017".to_string())
            .database("uncle_good_advice".to_string())
            .tls(false)
            .build()
            .unwrap();

        let _ = Storage::try_new(uri, "uncle_good_advice").await.unwrap();
    }

    #[tokio::test]
    async fn it_should_validate_store_implementation() -> Result<(), String> {
        use crate::shared::{SentimentDataBuilder, SentimentResultBuilder};
        use std::time::{SystemTime, UNIX_EPOCH};

        use super::{Storage, Uri, UriBuilder};
        use crate::traits::Store;

        let uri: Uri = UriBuilder::default()
            .url_with_credentials("localhost:27017".to_string())
            .database("uncle_good_advice".to_string())
            .tls(false)
            .build()
            .unwrap();

        let storage = Storage::try_new(uri, "uncle_good_advice").await.unwrap();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let result = SentimentResultBuilder::default()
            .negative(0.1)
            .positive(0.1)
            .neutral(0.9)
            .build()
            .unwrap_or_default();
        let collected = SentimentDataBuilder::default()
            .resource_id("fake_1233".to_string())
            .title("test".to_string())
            .origin("fake tweet".to_string())
            .text("This message is faked for test porpuses".to_string())
            .link("https://google.com".to_string())
            .created_at(now as u64)
            .coins(vec![])
            .keywords(vec![])
            .sentiment(result)
            .build()
            .unwrap_or_default();

        let id = storage.save(&collected).await?;

        let collected_result = storage.read_by_id(&id).await?;
        assert_eq!(collected, collected_result);

        Ok(())
    }
}
