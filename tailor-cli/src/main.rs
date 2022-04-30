extern crate clap;

use clap::{Args, Parser, Subcommand};



#[derive(Debug, Subcommand)]
enum Commands{
    #[clap(arg_required_else_help = true)]
    /// Initialize a new distributable server directory
    Init{
        // Name of the new folder to be created
        folder_name: String,
    },
    /// Generate a new server directory from a pre-existing example
    Example{
        /// Name of the example
        example_name: String,
    },
    /// Publish a server
    Publish,
}


/// CLI tool for the development and deployment of static and dynamic NFT metadata and servers.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}



fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Init{folder_name: n}      => init(&n),
        Commands::Example{example_name: n}   => example(&n),
        Commands::Publish   => publish(),
    };
}

fn init(name: &str) {

}

fn example(name: &str) {

}

fn publish() {

}