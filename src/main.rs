use inkstory::{instory, instory_to_ink, Diagram};
use lazy_static::lazy_static;
use regex::Regex;
use std::{error::Error, fs, path::PathBuf, str::FromStr};
use structopt::StructOpt;
use url::Url;

enum StoryLocator {
  Id(u32),
  File(PathBuf),
  Url(Url),
}

impl StoryLocator {
  fn get(&self) -> Result<Diagram, Box<dyn Error>> {
    fn download(url: &Url) -> Result<String, Box<dyn Error>> {
      Ok(ureq::get(&url.to_string()).call()?.into_string()?)
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
  type Err = Box<dyn Error>;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if let Ok(id) = s.parse() {
      return Ok(StoryLocator::Id(id));
    }

    if let Ok(url) = s.parse() {
      return Ok(StoryLocator::Url(url));
    }

    if let Ok(path) = s.parse() {
      return Ok(StoryLocator::File(path));
    }

    Err("Must be a story URL, a file path, or a story ID.".into())
  }
}

#[derive(StructOpt)]
struct Args {
  story_locator: StoryLocator,
}

#[paw::main]
fn main(args: Args) -> Result<(), Box<dyn Error>> {
  let diagram = args.story_locator.get()?;
  let story = instory_to_ink(&diagram)?;
  print!("{}", story);
  Ok(())
}
