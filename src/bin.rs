use structopt::StructOpt;
use std::fs;
use std::path::Path;
use chrono::prelude::*;
mod lib;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(parse(from_os_str))]
    source: std::path::PathBuf,
    #[structopt(short, long)]
    classic: bool,
}

/// usage: intbak -- <wow_source>
pub fn main() {
    let args = Cli::from_args();
    let mode = match args.classic {
        // TODO: verify class folder name
        true => "_classic_",
        false => "_retail_",
    };
    // supporting only retail for now
    let mut source_base = args.source.clone();
    source_base.push(mode.clone());

    let mut destination_base = args.source.clone();
    destination_base.push("interface_backups");
    destination_base.push(mode.clone());

    let snapshot = get_unix_timestamp_ms();

    let targets = vec!["Cache", "Interface", "WTF"];
    
    for entry in targets {
        let mut source = source_base.clone();
            source.push(entry);
        let mut destination = destination_base.clone();
            destination.push(&snapshot.to_string());
            destination.push(entry);
        let _r = lib::copy_directory_contents(&source, &destination);
    }
}

pub fn get_unix_timestamp_ms() -> i64 {
    let now = Utc::now();
    now.timestamp_millis()
}

