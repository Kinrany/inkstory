pub mod backend;

pub use backend::{Diagram, Response};
use backend::{Node, NodeKind};
use itertools::Itertools;
use std::{error::Error, fmt::Display};

#[derive(Clone, Debug)]
struct KnotName(String);

impl<T> From<T> for KnotName
where
  T: Into<String>,
{
  fn from(name: T) -> Self {
    KnotName(name.into())
  }
}

impl Display for KnotName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

struct Knot {
  name: KnotName,
  text: String,
  choices: Vec<(String, KnotName)>,
}

impl Display for Knot {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "=== {} ===", self.name)?;
    writeln!(f, "{}", self.text)?;
    self
      .choices
      .iter()
      .map(|(name, text)| writeln!(f, "+ [{}] -> {}", name, text))
      .collect::<Result<Vec<_>, _>>()?;
    Ok(())
  }
}

fn temp_knot_name(node: &Node) -> KnotName {
  format!("knot_{}", node.id).into()
}

pub fn generate_story(diagram: &Diagram) -> Result<String, Box<dyn Error>> {
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
    .map(|knot| knot.to_string());

  let story_start = format!("-> {}", start_knot_name);
  let story = vec![story_start]
    .into_iter()
    // TODO: make cleaner by implementing chain_back
    // so that the arguments are reversed.
    .chain(knots)
    .join("\n");
  Ok(story)
}
