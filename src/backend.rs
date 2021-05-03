//! Instory backend response types.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug, Serialize)]
pub struct StoryId(u32);

#[derive(Deserialize, Debug, Serialize)]
pub struct NodeId(Uuid);

#[derive(Deserialize, Debug, Serialize)]
pub struct PortId(Uuid);

#[derive(Deserialize, Debug, Serialize)]
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

#[derive(Deserialize, Debug, Serialize)]
pub struct Asset {
  pub id: Uuid,
  pub url: String,
}

#[non_exhaustive]
#[derive(Deserialize, Debug, Serialize)]
pub enum PortType {
  #[serde(rename = "out")]
  Out,
  #[serde(rename = "in")]
  In,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Port {
  pub id: PortId,
  pub name: String,
  pub r#type: PortType,
  pub position: u32,
  #[serde(rename = "nodeId")]
  pub node_id: NodeId,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct NodeContext {
  pub text: String,
  pub timeout: u32,
  #[serde(rename = "timeoutUnit")]
  pub timeout_unit: String,
  #[serde(rename = "timeoutPortId")]
  pub timeout_port_id: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "type")]
pub enum NodeKind {
  #[serde(rename = "start")]
  Start {},
  #[serde(rename = "text-choice")]
  TextChoice { context: Option<NodeContext> },
}

#[derive(Deserialize, Debug, Serialize)]
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

#[derive(Deserialize, Debug, Serialize)]
// #[serde(untagged)]
#[non_exhaustive]
pub struct Response<T> {
  // Success { status: bool, data: T },
  status: bool,
  data: T,
}
