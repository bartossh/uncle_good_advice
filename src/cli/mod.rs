use crate::advise::Advisor;
use crate::traits::{Advise, Configur, Handler};
use clap::Parser;
use inquire::Text;

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
pub struct ProgramRunner;

impl Handler for ProgramRunner {
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
