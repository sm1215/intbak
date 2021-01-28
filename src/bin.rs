use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(parse(from_os_str))]
    source: std::path::PathBuf,
}

// TODO: hookup user input so they can provide their own source
pub fn main() {
    let args = Cli::from_args();
    println!("args {:?}", args);
    // let filename = match args {
    //     "filename" => 
    // }
}