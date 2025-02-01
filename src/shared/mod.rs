use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/// The sentence being examined and calculation of sentiment for given sentence.
#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Getters, Default, Builder,
)]
pub struct SentimentCollected {
    origin: String,
    sentence: String,
    created_at: u64,
    result: SentimentResult,
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
