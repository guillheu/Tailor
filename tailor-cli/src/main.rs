extern crate clap;
extern crate reqwest;

use clap::{Parser, Subcommand};
use std::io;
use std::fs::File;




static GIT_URL: &'static str = "https://github.com/guillheu/Tailor";


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
    let result = match args.command {
        Commands::Init{folder_name: n}      => (init(&n)),
        Commands::Example{example_name: n}   => example(&n),
        Commands::Publish   => publish(),
    };
    if result.is_err() {
        println!("Invalid input : {:?}", result.err());
    }
}

fn init(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let download_url = format!("{}{}", GIT_URL, "/raw/main/examples/default.zip");
    let resp = reqwest::blocking::get(download_url)?.bytes()?;
    // let mut zip = File::create(name)?;
    // io::copy(&mut resp.bytes()?.as_ref(), &mut zip)?;
    // let mut file = zip =
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(resp))?;
    println!("{:#?}", zip.len());
    zip.extract(std::path::PathBuf::from(name))?;
    Ok(())
}

fn example(name: &str)  -> Result<(), Box<dyn std::error::Error>>{
    Ok(())
}

fn publish() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}