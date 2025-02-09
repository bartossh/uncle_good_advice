use std::collections::HashSet;

use crate::traits::ExtractionStrategy;
use aho_corasick::AhoCorasick;

#[derive(Debug, Clone)]
pub struct CoinExtractor {
    extractor: AhoCorasick,
    coins: Vec<String>,
}

impl CoinExtractor {
    pub fn try_new(coins: &Vec<String>) -> Result<Self, String> {
        let coins_to_match = [
            (' ', ' '),
            (' ', '.'),
            ('(', ','),
            ('(', ' '),
            (' ', ')'),
            ('(', ')'),
            ('[', ','),
            ('[', ' '),
            (' ', ']'),
            ('[', ']'),
        ]
        .iter()
        .fold(vec![], |acc, (start, stop)| {
            let mut next_acc = acc;
            next_acc.extend(
                coins
                    .iter()
                    .map(|c| format!("{start}{c}{stop}").to_string())
                    .collect::<Vec<String>>(),
            );
            next_acc
        });

        let extractor = AhoCorasick::builder()
            .ascii_case_insensitive(true)
            .build(coins_to_match)
            .map_err(|e| format!("{e}"))?;
        let coins = coins.clone();

        Ok(Self { extractor, coins })
    }
}

impl ExtractionStrategy<String, String> for CoinExtractor {
    fn extract(&self, data: &String) -> Vec<String> {
        self.extractor
            .find_iter(data)
            .map_while(|m| self.coins.get(m.pattern().as_usize() % self.coins.len()))
            .map(|s| s.to_string())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect()
    }
}
