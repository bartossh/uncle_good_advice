use crate::advise::Advisor;
use crate::newsdata_io::connector::NewsDataIoConnectorBuilder;
use crate::newsdata_io::extractor::CoinExtractor;
use crate::newsdata_io::validator::NewsDataIoLanguageValidator;
use crate::traits::{Advise, Configur, Fetcher, Handler};
use clap::Parser;
use dotenv::dotenv;
use inquire::Text;
use std::env;
use std::time::Duration;
use tokio::time::interval;

const PULL_MODEL_PROMPT: &str = r#"
Analyze the text sentiment and provide the result in JSON format like in this example:
{"negative": 0.0, "neutral": 0.0, "positive": 0.0} where 0.0 is a real value given as a float.
"#;

const INTERVAL_S: u64 = 24 * 60 * 60 / 200; // 200 queries per day

/// Handles the startup commands of a CLI program.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Prompt to the Advise provider to preset analyze parameters.
    #[arg(short, long)]
    prompt: String,
}

impl Configur for Args {
    fn prompt(&self) -> String {
        self.prompt.to_string()
    }
}

/// Handles the CLI commands and runs the program.
pub struct ChatRunner;

impl Handler for ChatRunner {
    async fn run(&self) -> Result<(), String> {
        let args = Args::parse();

        let mut advise = Advisor::try_new(args).await?;

        loop {
            let message = Text::new("Please write message to analyze.").prompt();

            let Ok(msg) = message else {
                return Err("Program was interrupted...".to_string());
            };
            let analyzed_msg = advise.advise_about(msg.as_ref()).await?;
            println!("{analyzed_msg}");
            println!("-------------");
        }
    }
}

/// Handles the PullModel execution.
pub struct PullModel;

impl Handler for PullModel {
    async fn run(&self) -> Result<(), String> {
        dotenv().map_err(|e| format!("{e}"))?;
        let api_key = env::var("NEWSDATA_IO").map_err(|e| format!("{e}"))?;

        let args = Args {
            prompt: PULL_MODEL_PROMPT.to_string(),
        };

        let mut advise = Advisor::try_new(args).await?;

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
            "dogecoin".to_string(),
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

        let news_data_connector = NewsDataIoConnectorBuilder::default()
            .api_key(api_key)
            .lang_validator(lang_validator)
            .coin_extractor(coin_extractor)
            .build()
            .map_err(|e| format!("{e}"))?;

        let mut interval = interval(Duration::from_secs(INTERVAL_S));
        loop {
            interval.tick().await;
            let results = news_data_connector.pull().await?;
            for r in results.iter() {
                let resp = advise
                    .advise_about(format!("{}\n{}", r.title(), r.text()).as_str())
                    .await?;
                println!("===============================[ NEXT ARTICLE DATA ]===============================");
                println!(
                    "Title:{}\nText:\n{}\nCoins: {:?}\nKeywords: {:?}\nLink: {:?}\n",
                    r.title(),
                    r.text(),
                    r.coins(),
                    r.keywords(),
                    r.link(),
                );
                println!("===============================[ NEXT ARTICLE DATA ]===============================\n");
                println!("_______________________________[ SENTIMENT ANALITICS ]_____________________________");
                println!("{resp}");
                println!("_______________________________[ SENTIMENT ANALITICS ]_____________________________");
                println!("\n\n");
            }
        }
    }
}
