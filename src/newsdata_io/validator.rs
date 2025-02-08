use crate::traits::ValidatorStrategy;
use aho_corasick::AhoCorasick;

#[derive(Debug, Clone)]
pub struct NewsDataIoLanguageValidator {
    validator: AhoCorasick,
}

impl NewsDataIoLanguageValidator {
    pub fn try_new(langs: &Vec<String>) -> Result<Self, String> {
        let validator = AhoCorasick::builder()
            .ascii_case_insensitive(true)
            .build(langs)
            .map_err(|e| format!("{e}"))?;

        Ok(Self { validator })
    }
}

impl ValidatorStrategy<String> for NewsDataIoLanguageValidator {
    fn is_valid(&self, value: &String) -> bool {
        self.validator.is_match(value)
    }
}
