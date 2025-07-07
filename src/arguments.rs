use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(about, version, author)]
pub struct KafkaReaderArgs {
    
    #[command(subcommand)]
    pub command: Option<KafkaReaderCommands>,
}

#[derive(Subcommand, Debug)]
pub enum KafkaReaderCommands {
    
    #[command()]
    Read(ReadArgs),

    #[command(subcommand)]
    Profile(ProfileSubcommands),
}

#[derive(Subcommand, Debug)]
pub enum ProfileSubcommands {
    Add(AddProfileArgs),
    Edit,
    Remove,
    List,
}

#[derive(Args, Debug)]
pub struct AddProfileArgs {

    /// Profile name
    #[arg(short, long, required = true)]
    pub profile: String,

    /// Broker address
    #[arg(short, long, required = false, default_value = "localhost:9092")]
    pub broker: String,

    /// Topic name
    #[arg(short, long, required = true)]
    pub topic: String,
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
