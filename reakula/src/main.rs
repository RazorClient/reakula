use clap::{Error as CliError, Parser};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Command::parse();

    Command::run(&cli).await?;

    std::future::pending().await
}


#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Command {
}

impl Command {
    async fn run(&self) -> Result<(), CliError> {
        println!("Running command");
        Ok(())
    }
}