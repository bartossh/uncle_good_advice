use crate::traits::ExtractionStrategy;
use aho_corasick::AhoCorasick;

#[derive(Debug, Clone)]
pub struct CoinExtractor {
    extractor: AhoCorasick,
    coins: Vec<String>,
}

impl CoinExtractor {
    pub fn try_new(coins: &Vec<String>) -> Result<Self, String> {
        let coins_space = coins
            .iter()
            .map(|c| format!(" {c} "))
            .collect::<Vec<String>>();
        let coins_dot = coins
            .iter()
            .map(|c| format!(" {c}."))
            .collect::<Vec<String>>();
        let coins_open_bracket = coins
            .iter()
            .map(|c| format!("({c}"))
            .collect::<Vec<String>>();
        let coins_close_bracket = coins
            .iter()
            .map(|c| format!("{c})"))
            .collect::<Vec<String>>();
        let coins_brackets = coins
            .iter()
            .map(|c| format!("({c})"))
            .collect::<Vec<String>>();
        let coins_brackets_space = coins
            .iter()
            .map(|c| format!("( {c} )"))
            .collect::<Vec<String>>();
        let mut coins_to_match = Vec::with_capacity(
            coins_space.len()
                + coins_dot.len()
                + coins_open_bracket.len()
                + coins_close_bracket.len()
                + coins_brackets.len()
                + coins_brackets_space.len(),
        );
        coins_to_match.extend(coins_space);
        coins_to_match.extend(coins_dot);

        let extractor = AhoCorasick::builder()
            .ascii_case_insensitive(true)
            .build(coins_to_match)
            .map_err(|e| format!("{e}"))?;

        let mut coins_ids = coins.to_owned();
        coins_ids.extend(coins.clone());
        coins_ids.extend(coins.clone());
        coins_ids.extend(coins.clone());
        coins_ids.extend(coins.clone());
        coins_ids.extend(coins.clone());

        Ok(Self {
            extractor,
            coins: coins_ids,
        })
    }
}

impl ExtractionStrategy<String, String> for CoinExtractor {
    fn extract(&self, data: &String) -> Vec<String> {
        self.extractor
            .find_iter(data)
            .map_while(|m| self.coins.get(m.pattern().as_usize()))
            .map(|s| s.to_string())
            .collect()
    }
}
