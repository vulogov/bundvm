extern crate log;

use std::env;
use clap::{Args, Parser, Subcommand};

pub mod setloglevel;

pub mod bund_version;

use crate::stdlib;

pub fn init() {
    let cli = Cli::parse();
    setloglevel::setloglevel(&cli);
    stdlib::initlib(&cli);

     match &cli.command {
        Commands::Version(_) => {
            log::debug!("Get the tool version");
            bund_version::run(&cli);
        }
     }
}

#[derive(Parser, Clone)]
#[clap(name = "bundvm")]
#[clap(author = "Vladimir Ulogov <vladimir@ulogov.us>")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "Virtual machine for TwoStack based BUND language", long_about = None)]
pub struct Cli {
    #[clap(short, long, action = clap::ArgAction::Count, help="Increase verbosity")]
    pub debug: u8,

    #[clap(help="ZENOH bus address", long, default_value_t = String::from(env::var("ZBUS_ADDRESS").unwrap_or("tcp/127.0.0.1:7447".to_string())))]
    pub bus: String,

    #[clap(help="ZENOH listen address", long, default_value_t = String::from_utf8(vec![]).unwrap())]
    pub listen: String,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Disable multicast discovery of ZENOH bus")]
    pub disable_multicast_scout: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Configure CONNECT mode for ZENOH bus")]
    pub set_connect_mode: bool,

    #[clap(short, long, default_value_t = 32, help="Number of threads allocated to a thread manager")]
    pub n: usize,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Version(Version),
}

#[derive(Args, Clone, Debug)]
#[clap(about="Get the version of the VM")]
struct Version {
    #[clap(last = true)]
    args: Vec<String>,
}
