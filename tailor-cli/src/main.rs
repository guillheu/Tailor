extern crate clap;
extern crate reqwest;

use clap::{Parser, Subcommand};
use std::fs::copy;
use std::env::current_exe;
use std::path::PathBuf;



static GIT_URL: &'static str = "https://github.com/guillheu/Tailor";


#[derive(Debug, Subcommand)]
enum Commands{
    #[clap(arg_required_else_help = true)]
    /// Initialize a new distributable server directory.
    /// To then run the server, simply run the tailor-server-redis executable.
    /// On UNIX systems, add executable permissions to tailor-server-redis
    Init{
        // Name of the new folder to be created
        folder_name: String,
    },
    /// Generate a new server directory from a pre-existing example
    /// To then run the server, simply run the tailor-server-redis executable.
    /// On UNIX systems, add executable permissions to tailor-server-redis
    Example{
        /// Name of the example
        example_name: String,
    },
    /// Publish a server
    /// NOT YET IMPLEMENTED
    /// Eventually will allow to publish metadata and NFTs to either Aleph, IPFS or Arweave
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
    build_project_from_example("default", name)?;
    Ok(())
}

fn example(name: &str)  -> Result<(), Box<dyn std::error::Error>>{
    build_project_from_example(name, name)?;
    Ok(())
}

fn publish() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn build_project_from_example(example_name: &str, target_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    //DLing & extracting example zip
    println!("Downloading example \"{}\" from {}", example_name, GIT_URL);
    let download_url = format!("{}{}{}{}", GIT_URL, "/raw/main/examples/", example_name, "/content.zip");
    let resp = reqwest::blocking::get(download_url)?.bytes()?;
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(resp))?;
    let mut target_path = PathBuf::from(target_folder);
    println!("Extracting into {}...", target_folder);
    zip.extract(target_path.clone())?;


    //Copying server binary
    println!("Copying server binary...");
    let mut source_path = current_exe()?;
    let tailor_server_path = PathBuf::from("tailor-server");
    source_path.pop();
    source_path.push(tailor_server_path.clone());
    target_path.push(tailor_server_path);
    println!("{:?} : {:?}", source_path, target_path);
    copy(source_path, target_path)?;
    Ok(())
}