use structopt::StructOpt;
use std::fs;
use std::path::{Path, PathBuf};
use chrono::prelude::*;
mod lib;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(parse(from_os_str))]
    source: Option<PathBuf>,
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
struct Snapshot {
    when: i64,
    source: PathBuf,
    destination: PathBuf,
    targets: Vec<String>,
}

/// Details describing the backup
impl Snapshot {
    fn new() -> Self {
        Self {
            when: Self::when(),
            source: PathBuf::from("Z:\\World of Warcraft"),
            destination: PathBuf::from("interface_backups"),
            targets: vec![
                String::from("Cache"),
                String::from("Interface"),
                String::from("WTF"),
            ],
        }
    }
    fn when() -> i64 {
        let now = Utc::now();
        now.timestamp_millis()
    }
}

/// usage: intbak -- <wow_source>
pub fn main() {
    println!("interface backup running. . . ");
    let snapshot = Snapshot::new();
    let args = Cli::from_args();
    let mut source_base = match args.source {
        Some(source) => source,
        _ => snapshot.source
    };
    
    source_base.push(Mode::mode_path());

    let mut destination_base = source_base.clone();
    destination_base.push(snapshot.destination);
    destination_base.push(Mode::mode_path());   
    destination_base.push(snapshot.when.to_string());

    // TODO: setup these as a default,
    // optionally accept args from cli
    let targets = vec!["Cache", "Interface", "WTF"];

    lib::run_backup(source_base, destination_base, targets, args.debug);
    println!("complete.");
}

