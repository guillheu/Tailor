extern crate clap;
extern crate reqwest;
extern crate rs_docker;

use clap::{Parser, Subcommand};
use rs_docker::Docker;
use rs_docker::container::{ContainerCreate, HostConfigCreate};
use std::fs::copy;
use std::env::current_exe;
use std::path::PathBuf;


static TAILOR_SERVER_DOCKER_REPO: &'static str = "tailor-server";
static TAILOR_SERVER_DEFAULT_DOCKER_TAG: &'static str = "dev";
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
    Debug,
}


/// CLI tool for the development and deployment of static and dynamic NFT metadata and servers.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}



fn debug() -> Result<(), Box<dyn std::error::Error>> {
    let mut docker = Docker::connect("unix:///var/run/docker.sock")?;
    docker.create_image(TAILOR_SERVER_DOCKER_REPO, TAILOR_SERVER_DEFAULT_DOCKER_TAG)?;
    let exposed_ports = docker.inspect_image(&format!("{}:{}", TAILOR_SERVER_DOCKER_REPO, TAILOR_SERVER_DEFAULT_DOCKER_TAG))?.ContainerConfig.ExposedPorts.unwrap();
    println!("{:#?}", exposed_ports);
    // let host_config_create = HostConfigCreate{
    //     NetworkMode: Some("bridge".to_string()),
    //     PublishAllPorts: Some(false),

    // }
    // let container_create = ContainerCreate{
    //     Image: format!("{}:{}", TAILOR_SERVER_DOCKER_REPO, TAILOR_SERVER_DEFAULT_DOCKER_TAG),
    //     Labels: None,
    //     ExposedPorts: None,
    //     HostConfig: None,
    // };
    // docker.create_container("testing".to_string(), container_create)?;
    // docker.start_container("testing")?;
    Ok(())
}

fn main() {

    let args = Cli::parse();
    let result = match args.command {
        Commands::Init{folder_name: n}      => (init(&n)),
        Commands::Example{example_name: n}   => example(&n),
        Commands::Publish   => publish(),
        Commands::Debug     => debug(),
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