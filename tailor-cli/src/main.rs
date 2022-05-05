extern crate clap;
extern crate reqwest;
extern crate rs_docker;

use clap::{Parser, Subcommand};
use rs_docker::Docker;
use rs_docker::container::{ContainerCreate, HostConfigCreate, PortBinding, Mount};
use std::fs::{create_dir_all};
use std::env::{current_dir};
use std::path::{PathBuf, Path};
use std::collections::HashMap;


static TAILOR_SERVER_DOCKER_REPO: &'static str = "guillh/tailor-server";
static TAILOR_SERVER_DEFAULT_DOCKER_TAG: &'static str = "0.1.1";


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
    // Start,
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
        println!("Error: {}", result.err().unwrap().as_ref());
    }
    else {
        println!("Success!");
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
    println!("Creating {} directory...", target_folder);
    if Path::new(target_folder).is_dir() {

        return Result::Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "directory already exists".to_string())));
    }
    create_dir_all(PathBuf::from(target_folder.clone()))?;

    run_container(None, Some(vec![Mount{
        Target: "/mnt".to_string(),
        Source: format!("{}{}{}", current_dir()?.to_str().unwrap(), "/", target_folder),
        Type: "bind".to_string(),
        ReadOnly: false,
        Consistency: None,
        BindOptions: None,
        VolumeOptions: None,
        TmpfsOptions: None,
    }]), Some(vec!["/bin/sh".to_string(), "-c".to_string(), format!("{}{}{}", "cp -r ../examples/", example_name, "/* /mnt")]))?;
    Ok(())
}



fn run_container(port_binding: Option<PortBinding>, mounts: Option<Vec<Mount>>, entrypoint_override: Option<Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
    let container_name = String::from("tailor-server_managed");
    println!("Connecting to docker daemon...");
    let mut docker = Docker::connect("unix:///var/run/docker.sock")?;
    println!("Fetching image {}:{}", TAILOR_SERVER_DOCKER_REPO, TAILOR_SERVER_DEFAULT_DOCKER_TAG);
    docker.create_image(TAILOR_SERVER_DOCKER_REPO, TAILOR_SERVER_DEFAULT_DOCKER_TAG)?;


    let port_bindings = match port_binding {
        Some(binding)  => {
            let mut port_bindings = HashMap::<String, Vec<PortBinding>>::new();
            let exposed_ports = docker.inspect_image(&format!("{}:{}", TAILOR_SERVER_DOCKER_REPO, TAILOR_SERVER_DEFAULT_DOCKER_TAG))?.ContainerConfig.ExposedPorts.unwrap();
            port_bindings.insert(exposed_ports.keys().last().unwrap().clone(), vec![binding]);
            Some(port_bindings)
        },
        None        => None,
    };

    let host_config_create = HostConfigCreate{
        NetworkMode: Some("bridge".to_string()),
        PublishAllPorts: Some(false),
        PortBindings: port_bindings,
        AutoRemove: true,
        Mounts: mounts,
    };
    let container_create = ContainerCreate{
        Image: format!("{}:{}", TAILOR_SERVER_DOCKER_REPO, TAILOR_SERVER_DEFAULT_DOCKER_TAG),
        Labels: None,
        ExposedPorts: None,
        HostConfig: Some(host_config_create),
        Entrypoint: entrypoint_override,
    };
    println!("Creating ephemeral container {}", &container_name);
    docker.create_container(container_name.clone(), container_create)?;
    println!("Starting container {}", &container_name);
    docker.start_container(&container_name)?;
    Ok(())
}