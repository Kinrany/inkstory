use std::fmt::Display;

#[derive(Clone, Debug)]
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
  pub name: KnotName,
  pub text: String,
  pub choices: Vec<(String, KnotName)>,
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

pub struct Story {
  pub start: KnotName,
  pub knots: Vec<Knot>,
}

impl Display for Story {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "-> {}", self.start)?;
    self
      .knots
      .iter()
      .map(|knot| writeln!(f, "{}", knot))
      .collect::<Result<Vec<_>, _>>()?;
    Ok(())
  }
}
