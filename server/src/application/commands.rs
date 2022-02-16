use super::*;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage configurations
    Configure {
        #[clap(subcommand)]
        op: ConfigureOperations,
    },
    /// Start the server
    Start,
}

#[derive(Subcommand, Debug)]
pub enum ConfigureOperations {
    /// Prints the value corresponding a given key
    Get {
        /// The configuration keypath of the desired value
        keypath: Option<String>,
    },
    /// List all configuration entries
    List,
}
