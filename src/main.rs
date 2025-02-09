#[cfg(feature = "chatmodel")]
use uncle_good_advice_lib::{cli::ChatRunner, traits::Handler};
#[cfg(feature = "pullmodel")]
use uncle_good_advice_lib::{cli::PullModel, traits::Handler};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    run().await;
}

#[cfg(feature = "pullmodel")]
async fn run() {
    let program_runner = PullModel;
    let Err(e) = program_runner.run().await else {
        return;
    };
    println!("{e}");
}

#[cfg(feature = "chatmodel")]
async fn run() {
    let program_runner = ChatRunner;
    let Err(e) = program_runner.run().await else {
        return;
    };
    println!("{e}");
}
