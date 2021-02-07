use structopt::StructOpt;
use std::fs;
use std::path::Path;
use chrono::prelude::*;
mod lib;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(default_value = "z:/World of Warcraft", parse(from_os_str))]
    source: std::path::PathBuf,
    #[structopt(short, long)]
    classic: bool,
    #[structopt(short, long)]
    debug: bool,
    // #[] //TODO: Possible to input a list for additional target dirs?
}

#[derive(Debug)]
struct Mode {
    is_classic: bool,
}

/// Whether we should target classic or retail version
impl Mode {
    fn mode_path() -> String {
        match Cli::from_args().classic {
            // TODO: verify classic folder name
            true => String::from("_classic_"),
            false => String::from("_retail_"),
        }
    }
}

#[derive(Debug)]
// TODO: move source, destination here
struct Snapshot {
    when: i64,
}

/// Describes the moment the backup is made
impl Snapshot {
    fn when() -> i64 {
        let now = Utc::now();
        now.timestamp_millis()
    }

}

/// usage: intbak -- <wow_source>
pub fn main() {
    println!("interface backup running. . . ");
    let args = Cli::from_args();
    let mut source_base = args.source.clone();
    source_base.push(Mode::mode_path());

    let mut destination_base = args.source.clone();
    destination_base.push("interface_backups");
    destination_base.push(Mode::mode_path());

    let snapshot = Snapshot::when();
    destination_base.push(&snapshot.to_string());

    // TODO: setup these as a default,
    // optionally accept args from cli
    let targets = vec!["Cache", "Interface", "WTF"];

    // TODO: pass a Snapshot over after source and dest are moved
    lib::run_backup(source_base, destination_base, targets, args.debug);
    println!("complete.");
}

