use structopt::StructOpt;
use std::{io::{Read, Result}, path::Path, fs::File};
#[derive(Debug, StructOpt)]
struct Cli {
    /// The path to the file to compress
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn open_file(path: &Path) -> File {
    let f = File::open(path);
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!(format!("can't load file {}", error)),
    };
    f
}

fn buffer_file(mut file: &File) -> Result<Vec<u8>>{
    let mut tmp: Vec<u8> = Vec::new();
    file.read_to_end(&mut tmp)?;
    Ok(tmp)
}

fn main() {
    
    let args = Cli::from_args();
    let path = Path::new(args.path.as_path());
    println!("path: {:?} exits: {:?}!", path, path.exists());
    let file: File = open_file(path);
    let res = buffer_file(&file);
    let buf = match res {
        Ok(a) => a,
        _ => unreachable!(),
    };
    print!("buf {:?}", buf);

    // TODO: read source file
    // TODO: compress or decompress
    // TODO: write compressed file 
}
