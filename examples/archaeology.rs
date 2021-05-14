//! Request and parse a story about archaeology students by Fantom_18.

use anyhow::{anyhow, Error};
use fehler::throws;
use instory::{instory_to_ink, Diagram, Response as InstoryResponse};

#[throws()]
fn main() {
  let instory: InstoryResponse<Diagram> =
    ureq::get("https://api.instory.su/api/stories/17801/diagram")
      .call()?
      .into_json()?;

  let diagram = instory.data;
  let story = instory_to_ink(&diagram).map_err(|_| anyhow!("failed to generate"))?;

  println!("{}", story);
}
