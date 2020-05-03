/// Glyphs Implementation
///
/// Defines glyph as core of Godwit's datatype of choice. Implementations support single borrow lifecyle and must be serializable.
///
/// Structs/Enums:
///    Glyph -> Define structure and specification for Godwit glyphs.
///
/// Implementation methods:
///    to_str -> Convert glyph to generic string representation.
///
/// Functions:
///    new -> Creates and returns a Glyph using tag and id as parameters.
///    from -> Creates and returns a Glyph using single string representation parameter.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Glyph {
    pub tag: String,
    pub id: String
}

impl Glyph {
    pub fn to_str(self: &Self) -> String {
        return format!("@{}/{}", self.tag, self.id);
    }
}

impl Default for Glyph {
  fn default() -> Self {
      new("default", "default")
  }
}

pub fn from(glyph_str: String) -> Result<Glyph, &'static str> {
    let vec = glyph_str.split('/').collect::<Vec<&str>>();

    let tag: &str;
    let id: &str;

    if let Some(value) = vec.get(0) {
        tag = value;
    }
    else {
        return Err("Glyph name is invalid.");
    }

    if let Some(value) = vec.get(1) {
        id = value;
    }
    else {
        return Err("Glyph name is invalid.");
    }

    if !tag.starts_with("@") || tag.is_empty() || id.is_empty() {
        Err("Glyph name is invalid.")
    }
    else {
        Ok(new(&tag[1..], id))
    }
}

pub fn new(tag: &str, id: &str) -> Glyph {
    return Glyph {
        tag: String::from(tag),
        id: String::from(id)
    }
}
