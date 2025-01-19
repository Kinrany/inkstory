use std::{
  collections::{HashMap, HashSet, VecDeque},
  fmt::Display,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct KnotName(String);

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

pub struct Knot {
  pub text: String,
  pub choices: Vec<(String, KnotName)>,
}

impl Display for Knot {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if !self.text.is_empty() {
      writeln!(f, "{}", self.text)?;
    }

    if !self.choices.is_empty() {
      self
        .choices
        .iter()
        .map(|(name, text)| writeln!(f, "+ [{}] -> {}", name, text))
        .collect::<Result<Vec<_>, _>>()?;
    } else {
      writeln!(f, "-> END")?;
    }
    Ok(())
  }
}

pub struct Story {
  pub start: KnotName,
  pub knots: HashMap<KnotName, Knot>,
}

impl Display for Story {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut queue = VecDeque::new();
    let mut unreached_knots = self.knots.keys().collect::<HashSet<_>>();

    // Start with an immediate divert to the first knot
    writeln!(f, "-> {}", self.start)?;
    queue.push_back(&self.start);

    // Write the knots in the order they happen in the story
    while let Some(name) = queue.pop_front() {
      // Mark knot as reached.
      // If it has already been reached before, skip.
      if unreached_knots.remove(&name) {
        // Write the knot, add choices to the knot queue
        writeln!(f, "=== {} ===", name)?;
        if let Some(knot) = self.knots.get(name) {
          writeln!(f, "{}", knot)?;
          for (_, choice) in &knot.choices {
            queue.push_back(choice);
          }
        } else {
          writeln!(f, "// knot not found")?;
        }
      }
    }

    // Write the knots not reachable from the first one, if any
    if !unreached_knots.is_empty() {
      writeln!(f, "// Some knots are unreachable:")?;
      for name in unreached_knots {
        writeln!(f, "=== {} ===", name)?;
        if let Some(knot) = self.knots.get(name) {
          writeln!(f, "{}", knot)?;
        } else {
          writeln!(f, "// knot not found")?;
        }
      }
    }

    Ok(())
  }
}
