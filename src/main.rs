use uncle_good_advice_lib::{cli::ProgramRunner, traits::Handler};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let program_runner = ProgramRunner;
    let Err(e) = program_runner.run().await else {
        return;
    };
    println!("{e}");
}
