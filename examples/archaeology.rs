//! Request and parse a story about archaeology students by Fantom_18.

use anyhow::Error;
use fehler::throws;
use instory::{Diagram, Response as InstoryResponse};

#[throws()]
fn main() {
  let instory: InstoryResponse<Diagram> =
    ureq::get("https://api.instory.su/api/stories/17801/diagram")
      .call()?
      .into_json()?;

  println!("{:?}", instory);
}
