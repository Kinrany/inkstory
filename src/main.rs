use inkstory::Inkstory;
use std::error::Error;

#[paw::main]
fn main(args: Inkstory) -> Result<(), Box<dyn Error>> {
  args.exec()
}
