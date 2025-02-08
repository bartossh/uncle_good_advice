use super::{extractor::CoinExtractor, validator::NewsDataIoLanguageValidator};
use crate::{
    shared::{NewsResponse, NewsResponseBuilder},
    traits::{ExtractionStrategy, Fetcher, ValidatorStrategy},
};
use derive_builder::Builder;
use reqwest;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    time::{SystemTime, UNIX_EPOCH},
};

const ORIGIN_URL: &str = "https://newsdata.io";
const NEWSDATAIO_URL: &str = "https://newsdata.io/api/1/latest?apikey=";
const SUCCESS: &str = "success";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NewsDataIoArticle {
    article_id: String,
    title: Option<String>,
    link: Option<String>,
    keywords: Option<Vec<String>>,
    creator: Option<Vec<String>>,
    video_url: Option<String>,
    description: Option<String>,
    content: Option<String>,
    #[serde(rename = "pubDate")]
    pub_data: Option<String>,
    #[serde(rename = "pubDataTZ")]
    pub_date_tz: Option<String>,
    image_url: Option<String>,
    source_id: Option<String>,
    source_priority: Option<u32>,
    source_name: Option<String>,
    source_url: Option<String>,
    source_icon: Option<String>,
    language: Option<String>,
    country: Option<Vec<String>>,
    category: Option<Vec<String>>,
    ai_tag: Option<String>,
    ai_org: Option<String>,
    ai_region: Option<String>,
    duplicate: bool,
    sentiment: Option<String>,
    sentiment_stats: Option<String>,
    coins: Option<Vec<String>>,
}

impl TryInto<NewsResponse> for NewsDataIoArticle {
    type Error = String;

    fn try_into(self) -> Result<NewsResponse, Self::Error> {
        let now = SystemTime::now();
        NewsResponseBuilder::default()
            .id(self.article_id)
            .title(self.title.unwrap_or_default())
            .origin(ORIGIN_URL.to_string())
            .text(self.description.unwrap_or_default())
            .link(self.link.unwrap_or_default())
            .created_at(
                now.duration_since(UNIX_EPOCH)
                    .map_err(|e| format!("{e}"))?
                    .as_millis() as u64,
            )
            .coins(self.coins.unwrap_or_default())
            .keywords(self.keywords.unwrap_or_default())
            .build()
            .map_err(|e| format!("{e}"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NewsDataIoResponse {
    status: String,
    #[serde(rename = "totalResults")]
    total_results: u32,
    results: Vec<NewsDataIoArticle>,
    #[serde(rename = "nextPage")]
    next_page: String,
}

#[derive(Debug, Clone, Builder)]
pub struct NewsDataIoConnector<V, E>
where
    V: ValidatorStrategy<String>,
    E: ExtractionStrategy<String, String>,
{
    api_key: String,
    lang_validator: V,
    coin_extractor: E,
}

impl<'a> Fetcher<'a, NewsResponse>
    for NewsDataIoConnector<NewsDataIoLanguageValidator, CoinExtractor>
{
    async fn pull(&self) -> Result<Vec<NewsResponse>, String> {
        let mut response = self
            .fetch_latest_crypto()
            .await
            .map_err(|e| format!("{e}"))?;

        println!("STATUS: {}", response.status);
        if response.status.as_str() != SUCCESS {
            return Err(format!("Response status: {}", response.status));
        }

        let news = response
            .results
            .iter_mut()
            .filter(|res| {
                if let Some(lang) = res.language.borrow() {
                    self.lang_validator.is_valid(lang)
                } else {
                    false
                }
            })
            .map(|n| {
                n.coins = Some([n.title.borrow(), n.description.borrow()].iter().fold(
                    vec![],
                    |acc, s| {
                        if let Some(s) = s {
                            let mut extracted = self.coin_extractor.extract(s);
                            extracted.extend(acc);
                            return extracted;
                        }
                        acc
                    },
                ));
                n
            })
            .map(|res| res.to_owned().try_into())
            .collect::<Result<Vec<NewsResponse>, String>>()?;

        Ok(news)
    }
}

impl NewsDataIoConnector<NewsDataIoLanguageValidator, CoinExtractor> {
    async fn fetch_latest_crypto(&self) -> Result<NewsDataIoResponse, Box<dyn std::error::Error>> {
        let resp = reqwest::get(format!("{NEWSDATAIO_URL}{}&q=crypto", self.api_key))
            .await?
            .json::<NewsDataIoResponse>()
            .await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn it_should_call_newsdata_io_for_crypto_news() -> Result<(), String> {
        use super::*;
        use crate::traits::Fetcher;
        use dotenv::dotenv;
        use std::env;

        dotenv().map_err(|e| format!("{e}"))?;

        let api_key = env::var("NEWSDATA_IO").map_err(|e| format!("{e}"))?;
        let lang_validator = NewsDataIoLanguageValidator::try_new(&vec![
            "english".to_string(),
            "eng".to_string(),
            "british".to_string(),
        ])?;

        let coin_extractor = CoinExtractor::try_new(&vec![
            "stablecoin".to_string(),
            "stablecoins".to_string(),
            "nft".to_string(),
            "nfts".to_string(),
            "bitcoin".to_string(),
            "ethereum".to_string(),
            "solana".to_string(),
            "bitcoins".to_string(),
            "ethereums".to_string(),
            "solanas".to_string(),
            "sol".to_string(),
            "btc".to_string(),
            "eth".to_string(),
            "sol".to_string(),
            "usdt".to_string(),
            "xrp".to_string(),
            "bnb".to_string(),
            "usdc".to_string(),
            "dodge".to_string(),
            "doge".to_string(),
            "ada".to_string(),
            "steth".to_string(),
        ])?;

        let news_data_connector = NewsDataIoConnectorBuilder::create_empty()
            .api_key(api_key)
            .lang_validator(lang_validator)
            .coin_extractor(coin_extractor)
            .build()
            .map_err(|e| format!("{e}"))?;

        let news = news_data_connector.pull().await?;

        news.iter().for_each(|n| println!("{n:?}\n"));

        Ok(())
    }
}
