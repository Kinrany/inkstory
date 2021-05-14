use anyhow::Result;
use inkstory::Inkstory;

#[paw::main]
fn main(args: Inkstory) -> Result<()> {
  args.exec()
}
