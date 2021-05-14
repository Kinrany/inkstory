//! Instory backend response types.

use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct StoryId(u32);

#[derive(Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct NodeId(Uuid);

impl Display for NodeId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = str::replace(&self.0.to_string(), "-", "_");
    write!(f, "{}", s)
  }
}

#[derive(Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct PortId(Uuid);

#[derive(Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct EdgeId(Uuid);

#[derive(Deserialize, Debug, Serialize)]
struct User {
  pub id: u32,
  pub name: String,
  #[serde(rename = "displayName")]
  pub display_name: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct Options {
  #[serde(rename = "advancedMode")]
  pub advanced_mode: bool,
  #[serde(rename = "timeOfPassage")]
  pub time_of_passage: u32,
  #[serde(rename = "showTimeOfPassage")]
  pub show_time_of_passage: bool,
  #[serde(rename = "showNodeTimeout")]
  pub show_node_timeout: bool,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Variable;

#[derive(Deserialize, Debug, Serialize)]
pub struct Attachment;

#[derive(Deserialize, Debug, Serialize)]
pub struct Group;

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct Asset {
  pub id: Uuid,
  pub url: String,
}

#[non_exhaustive]
#[derive(Deserialize, Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum PortType {
  #[serde(rename = "out")]
  Out,
  #[serde(rename = "in")]
  In,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct Port {
  pub id: PortId,
  pub name: String,
  pub r#type: PortType,
  pub position: u32,
  #[serde(rename = "nodeId")]
  pub node_id: NodeId,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NodeContext {
  pub text: String,
  pub timeout: u32,
  #[serde(rename = "timeoutUnit")]
  pub timeout_unit: String,
  #[serde(rename = "timeoutPortId")]
  pub timeout_port_id: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum NodeKind {
  #[serde(rename = "start")]
  Start,
  #[serde(rename = "text-choice")]
  TextChoice { context: Option<NodeContext> },
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct Node {
  pub id: NodeId,
  #[serde(flatten)]
  pub kind: NodeKind,
  pub title: String,
  pub x: i32,
  pub y: i32,
  #[serde(rename = "storyId")]
  pub story_id: StoryId,
  pub background: Asset,
  pub audio: Asset,
  pub ports: Vec<Port>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Edge {
  pub id: EdgeId,
  pub source: Port,
  #[serde(rename = "sourceId")]
  pub source_id: PortId,
  pub target: Port,
  #[serde(rename = "targetId")]
  pub target_id: PortId,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Diagram {
  pub variables: Vec<Variable>,
  pub attachments: Vec<Attachment>,
  pub nodes: Vec<Node>,
  pub edges: Vec<Edge>,
  pub groups: Vec<Group>,
}

impl Diagram {
  pub fn choices<'a>(&self, node: &'a Node) -> Vec<(&'a String, &Node)> {
    // collect pairs (outgoing port, choice option text)
    let outgoing_port_ids = node
      .ports
      .iter()
      .filter_map(|port| match port.r#type {
        PortType::In => None,
        // port.name contains the text displayed for the choice option
        PortType::Out => Some((port.id, &port.name)),
      })
      .collect::<HashMap<_, _>>();

    // follow the edges to replace outgoing ports with incoming ports
    let incoming_port_ids = self
      .edges
      .iter()
      .filter_map(|edge| {
        outgoing_port_ids
          .get(&edge.source_id)
          .map(|&choice_option_text| (edge.target_id, choice_option_text))
      })
      .collect::<HashMap<_, _>>();

    self
      .nodes
      .iter()
      .map(|node| {
        node
          .ports
          .iter()
          .filter_map(|port| {
            incoming_port_ids
              .get(&port.id)
              .map(|&choice_option_text| (choice_option_text, node))
          })
          .collect::<Vec<_>>()
      })
      .flatten()
      .collect()
  }
}

#[derive(Deserialize, Debug, Serialize)]
// #[serde(untagged)]
#[non_exhaustive]
pub struct Response<T> {
  // should be an enum with Success { status: bool, data: T },
  status: bool,
  pub data: T,
}
