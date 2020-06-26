use structopt::StructOpt;
use std::path::Path;

#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to compress
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    
    let args = Cli::from_args();
    let path = Path::new(args.path.as_path());
    println!("path: {:?} exits: {:?}!", path, path.exists());
    
    // TODO: read source file
    // TODO: compress 
    // TODO: write compressed file 
}
