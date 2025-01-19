pub mod ink;
pub mod instory;

use anyhow::{anyhow, Error, Result};
pub use ink::Story;
use ink::{Knot, KnotName};
pub use instory::{Diagram, Response};
use instory::{Node, NodeKind};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, path::PathBuf, str::FromStr};
use structopt::StructOpt;
use url::Url;

/// Creates a closure that will count characters in the passed strings
/// and return `false` once the total count reaches `n`.
fn char_count_less_than(n: usize) -> impl FnMut(&String) -> bool {
  let mut count = 0;
  move |s: &String| {
    count += s.chars().count();
    count < n
  }
}

/// Deterministically generates knot name based on an instory node
fn temp_knot_name(node: &Node) -> KnotName {
  match &node.kind {
    NodeKind::Start => "start".into(),
    NodeKind::TextChoice { context } => {
      let knot_name = context
        .clone()
        .expect("Text node with no context")
        .text
        .split(|ch: char| ch.is_whitespace() || ch.is_ascii_punctuation())
        .filter(|s| !s.is_empty())
        .map(|s| {
          s.chars()
            .filter(|ch| ch.is_alphanumeric())
            .collect::<String>()
        })
        .take_while(char_count_less_than(30))
        .join("_");
      let id = node.id.to_string().chars().take(6).collect::<String>();
      format!("{}_{}", knot_name, id).into()
    }
  }
}

pub fn instory_to_ink(diagram: &Diagram) -> Result<Story> {
  let start_node = diagram
    .nodes
    .iter()
    .find(|node| node.kind == NodeKind::Start)
    .ok_or_else(|| anyhow!("no start node"))?;
  let first_node = diagram
    .choices(start_node)
    .first()
    .ok_or_else(|| anyhow!("start node has no child nodes"))?
    .1;

  let start_knot_name: KnotName = temp_knot_name(first_node);
  let knots = diagram
    .nodes
    .iter()
    .filter_map(|node| match &node.kind {
      NodeKind::TextChoice { context } => {
        let knot = Knot {
          text: context
            .clone()
            .expect("context is never empty for text nodes")
            .text,
          choices: diagram
            .choices(node)
            .iter()
            .map(|(choice_option_text, choice_node)| {
              (choice_option_text.to_string(), temp_knot_name(choice_node))
            })
            .collect(),
        };
        let name = temp_knot_name(node);
        Some((name, knot))
      }
      _ => None,
    })
    .collect();

  Ok(Story {
    start: start_knot_name,
    knots,
  })
}

enum StoryLocator {
  Id(u32),
  File(PathBuf),
  Url(Url),
}

impl StoryLocator {
  fn get(&self) -> Result<Diagram> {
    fn download(url: &Url) -> Result<String> {
      Ok(ureq::get(url.as_ref()).call()?.into_string()?)
    }

    let json = match self {
      StoryLocator::File(file) => fs::read_to_string(file)?,
      StoryLocator::Id(id) => download(&instory::diagram_url_from_id(*id))?,
      StoryLocator::Url(url) => {
        lazy_static! {
          static ref RE: Regex =
            Regex::new(r"^https://instory.su/story/(?P<id>\d+)(/play)?").unwrap();
        }
        let url_string = url.clone().to_string();
        let captures = RE.captures(&url_string);
        if let Some(captures) = captures {
          let id = captures.name("id").unwrap().as_str().parse().unwrap();
          download(&instory::diagram_url_from_id(id))?
        } else {
          download(url)?
        }
      }
    };
    let response: instory::Response<instory::Diagram> = serde_json::from_str(&json)?;
    Ok(response.data)
  }
}

impl FromStr for StoryLocator {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    if let Ok(id) = s.parse() {
      return Ok(StoryLocator::Id(id));
    }

    if let Ok(url) = s.parse() {
      return Ok(StoryLocator::Url(url));
    }

    Ok(StoryLocator::File(PathBuf::from(s)))
  }
}

#[derive(StructOpt)]
pub struct Inkstory {
  story_locator: StoryLocator,
  #[structopt(long, help = "Replaces \"rn\" with newline")]
  fix_rn: bool,
}

impl Inkstory {
  pub fn new(locator: &str) -> Result<Self> {
    let inkstory = Inkstory {
      story_locator: locator.parse()?,
      fix_rn: false,
    };
    Ok(inkstory)
  }

  pub fn exec(&self) -> Result<()> {
    let diagram = self.story_locator.get()?;
    let mut story = instory_to_ink(&diagram)?;
    if self.fix_rn {
      story.knots.iter_mut().for_each(|(_, knot)| {
        knot.text = knot.text.replace("rn", "\n");
      });
    }
    print!("{}", story);
    Ok(())
  }
}
