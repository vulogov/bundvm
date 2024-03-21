extern crate log;
use crate::cmd;
use crate::stdlib::{getfile, vm_feed};

pub fn run(_c: &cmd::Cli, feed: cmd::Feed)  {
    log::trace!("zbus_feed::run() reached");

    if feed.source.stdin {
        log::debug!("Ready to get VM instructions from stdin");
        vm_feed::instructions(getfile::get_file_from_stdin(), feed.clone());
    } else {
        match &feed.source.file {
            Some(script_name) => {
                match getfile::get_file_from_file(script_name.trim().to_string()) {
                    Some(script) => vm_feed::instructions(script, feed.clone()),
                    None => log::error!("Instructions set is empty"),
                }
            }
            None => {
                match &feed.source.url {
                    Some(script_name) => {
                        match getfile::get_file_from_uri(script_name.trim().to_string()) {
                            Some(script) => vm_feed::instructions(script, feed.clone()),
                            None => log::error!("Instructions set is empty"),
                        }
                    }
                    None => {
                        match &feed.source.eval {
                            Some(script) => vm_feed::instructions(script.to_string(), feed.clone()),
                            None => {
                                log::debug!("Rolling down to reading VM instructions from stdin");
                                vm_feed::instructions(getfile::get_file_from_stdin(), feed.clone());
                            }
                        }
                    }
                }
            }
        }
    }
}
