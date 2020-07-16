//! Glyphs Implementation
//!
//! Defines glyph as core of Godwit's datatype of choice. Implementations
//! support borrow lifecyle and can be serialized.
use crate::errors::GlyphError;
use getter_derive::Getter;
use log::debug;
use serde::{Deserialize, Serialize};
use std::convert::{From, Into, TryFrom};
use std::fmt;
use std::str::FromStr;
use structopt::StructOpt;

// TODO: Convert this to trait

/// Structure and specification for Godwit glyphs.
#[derive(Clone, Debug, StructOpt, Serialize, Deserialize, PartialEq, Getter)]
#[serde(rename_all = "snake_case")]
pub struct Glyph {
	pub tag: String,
	pub id: String,
}

impl Default for Glyph {
	fn default() -> Self {
		Glyph {
			tag: Default::default(),
			id: Default::default(),
		}
	}
}

impl fmt::Display for Glyph {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "@{}/{}", &self.tag, &self.id)
	}
}

impl Into<String> for Glyph {
	fn into(self) -> String {
		format!("@{}/{}", self.tag, self.id)
	}
}

impl TryFrom<String> for Glyph {
	type Error = GlyphError;

	fn try_from(glyph_str: String) -> Result<Self, Self::Error> {
		let glyph_vec = glyph_str.trim().split("/").collect::<Vec<_>>();

		if glyph_vec.get(0).is_some() && glyph_vec.get(1).is_some() && glyph_vec.get(2).is_none() {
			let (tag, id) = (glyph_vec[0], glyph_vec[1]);

			if !tag.is_empty() && !id.is_empty() && tag.starts_with("@") {
				return Ok(Glyph {
					tag: String::from(&tag[1..]),
					id: id.to_string(),
				});
			}
		}

		debug!("Attempt to parse invalid glyph {}", glyph_str);
		Err(GlyphError::InvalidGlyph {
			glyph: glyph_str.into(),
		})
	}
}

impl FromStr for Glyph {
	type Err = GlyphError;

	fn from_str(glyph_str: &str) -> Result<Self, Self::Err> {
		let glyph_vec = glyph_str.trim().split("/").collect::<Vec<_>>();

		if glyph_vec.get(0).is_some() && glyph_vec.get(1).is_some() && glyph_vec.get(2).is_none() {
			let (tag, id) = (glyph_vec[0], glyph_vec[1]);

			if !tag.is_empty() && !id.is_empty() && tag.starts_with("@") {
				return Ok(Glyph {
					tag: String::from(&tag[1..]),
					id: id.to_string(),
				});
			}
		}

		debug!("Attempt to parse invalid glyph {}", glyph_str);
		Err(GlyphError::InvalidGlyph {
			glyph: glyph_str.into(),
		})
	}
}
