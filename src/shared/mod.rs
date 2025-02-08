use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/// The news response from the service.
#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Getters, Default, Builder,
)]
pub struct NewsResponse {
    id: String,
    title: String,
    origin: String,
    text: String,
    link: String,
    created_at: u64,
    coins: Vec<String>,
    keywords: Vec<String>,
}

/// The SentimentData contains data about the sentiment.
#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Getters, Default, Builder,
)]
pub struct SentimentData {
    resource_id: String,
    title: String,
    origin: String,
    text: String,
    link: String,
    created_at: u64,
    coins: Vec<String>,
    keywords: Vec<String>,
    sentiment: SentimentResult,
}

/// The sentiment callculation.
#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Getters, Default, Builder,
)]
pub struct SentimentResult {
    negative: f64,
    positive: f64,
    neutral: f64,
}
