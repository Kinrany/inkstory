pub mod ink;
pub mod instory;

pub use ink::Story;
use ink::{Knot, KnotName};
pub use instory::{Diagram, Response};
use instory::{Node, NodeKind};
use std::error::Error;

fn temp_knot_name(node: &Node) -> KnotName {
  format!("knot_{}", node.id).into()
}

pub fn instory_to_ink(diagram: &Diagram) -> Result<Story, Box<dyn Error>> {
  let start_node = diagram
    .nodes
    .iter()
    .find(|node| node.kind == NodeKind::Start)
    .ok_or("no start node")?;
  let first_node = diagram
    .choices(start_node)
    .first()
    .ok_or("start node has no child nodes")?
    .1;

  let start_knot_name: KnotName = temp_knot_name(&first_node);
  let knots = diagram
    .nodes
    .iter()
    .filter_map(|node| match &node.kind {
      NodeKind::TextChoice { context } => Some(Knot {
        name: temp_knot_name(node),
        text: context
          .clone()
          .expect("context is never empty for text nodes")
          .text,
        choices: diagram
          .choices(node)
          .iter()
          .map(|(choice_option_text, node)| (choice_option_text.to_string(), temp_knot_name(node)))
          .collect(),
      }),
      _ => None,
    })
    .collect();

  Ok(Story {
    start: start_knot_name,
    knots,
  })
}
