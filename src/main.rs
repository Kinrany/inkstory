use inkstory::{instory, instory_to_ink};
use std::{error::Error, fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
  file: PathBuf,
}

#[paw::main]
fn main(args: Args) -> Result<(), Box<dyn Error>> {
  let json = fs::read_to_string(args.file)?;
  let diagram: instory::Response<instory::Diagram> = serde_json::from_str(&json)?;
  let story = instory_to_ink(&diagram.data)?;
  print!("{}", story);
  Ok(())
}
