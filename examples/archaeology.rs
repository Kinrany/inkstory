//! Request and parse a story about archaeology students by Fantom_18.

fn main() -> anyhow::Result<()> {
  eprintln!("this example doesn't work");
  inkstory::Inkstory::new("https://api.instory.su/api/stories/17801/diagram")?.exec()
}
