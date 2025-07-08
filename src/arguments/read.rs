use crate::arguments::Commands;
use clap::{Args, Parser};

#[derive(Parser)]
#[command(about, version, author)]
pub struct ReaderArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Args, Debug)]
pub struct ReadArgs {
    /// Broker address
    #[arg(short, long, required = false)]
    pub broker: Option<String>,

    /// Topic name
    #[arg(short, long, required = false)]
    pub topic: Option<String>,

    /// Profile name to load configuration
    #[arg(short, long, required = false)]
    pub profile: Option<String>,
}
