use clap::{Args, Subcommand};

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
