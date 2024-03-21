extern crate log;

use std::env;
use std::str::FromStr;
use clap::{Args, Parser, Subcommand};
use zenoh::config::{Config, ConnectConfig, ListenConfig, EndPoint, WhatAmI};

pub mod setloglevel;

pub mod bund_version;
pub mod bund_vm;
pub mod bund_shell;

use crate::stdlib;

pub fn init() {
    let cli = Cli::parse();
    setloglevel::setloglevel(&cli);
    stdlib::initlib(&cli);

    let mut config =  Config::default();

    if cli.disable_multicast_scout.clone() {
        match config.scouting.multicast.set_enabled(Some(false)) {
            Ok(_) => { log::debug!("Multicast discovery disabled")}
            Err(err) => {
                log::error!("Failure in disabling multicast discovery: {:?}", err);
                return;
            }
        }
    }
    match EndPoint::from_str(&cli.bus) {
        Ok(zconn) => {
            log::debug!("ZENOH bus set to: {:?}", &zconn);
            let _ = config.set_connect(ConnectConfig::new(vec![zconn]).unwrap());
        }
        Err(err) => {
            log::error!("Failure in parsing connect address: {:?}", err);
            return;
        }
    }
    match EndPoint::from_str(&cli.listen) {
        Ok(zlisten) => {
            log::debug!("ZENOH listen set to: {:?}", &zlisten);
            let _ = config.set_listen(ListenConfig::new(vec![zlisten]).unwrap());
        }
        Err(_) => {
            log::debug!("ZENOH listen set to default");
        }
    }
    if cli.set_connect_mode {
        log::debug!("ZENOH configured in CONNECT mode");
        let _ = config.set_mode(Some(WhatAmI::Client));
    } else {
        log::debug!("ZENOH configured in PEER mode");
        let _ = config.set_mode(Some(WhatAmI::Peer));
    }
    if config.validate() {
        log::debug!("ZENOH config is OK");
    } else {
        log::error!("ZENOH config not OK");
        return;
    }

    match &cli.command {
        Commands::Shell(shell) => {
            log::debug!("Enter in VM shell");
            bund_shell::run(&cli, shell.clone());
        }
        Commands::Vm(vm) => {
            log::debug!("Execute VM instructions");
            bund_vm::run(&cli, vm.clone());
        }
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

    #[clap(help="ZENOH bus address", long, default_value_t = String::from(env::var("ZENOH_ADDRESS").unwrap_or("tcp/127.0.0.1:7447".to_string())))]
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
    Shell(Shell),
    Vm(Vm),
    Version(Version),
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = true, multiple = false)]
pub struct SrcArgGroup {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Take VM instructins from STDIN")]
    pub stdin: bool,

    #[clap(short, long, help="URL pointing to the file with VM instructions")]
    url: Option<String>,

    #[clap(short, long, help="Filename of the file with VM instructions (full path)")]
    file: Option<String>,

    #[clap(help="Eval instruction passed through command line", long, default_value_t = String::from(""))]
    pub eval: String,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Skip evaluation")]
    pub none: bool,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Enter in VM shell")]
pub struct Shell {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Disable colors in shell")]
    pub nocolor: bool,

    #[clap(flatten)]
    source: SrcArgGroup,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Get the version of the VM")]
struct Version {
    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Execute VM commands.")]
pub struct Vm {
    #[clap(flatten)]
    source: SrcArgGroup,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Drop into VM shell after execution")]
    pub shell: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Disable colors in shell")]
    pub nocolor: bool,

}
