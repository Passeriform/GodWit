//! Glyphs Implementation
//!
//! Defines glyph as core of Godwit's datatype of choice. Implementations
//! support borrow lifecyle and can be serialized.
use crate::errors::GlyphError;
use log::debug;
use serde::{Deserialize, Serialize};
use std::convert::{From, TryFrom};
use std::fmt;
use std::str::FromStr;
use structopt::StructOpt;

// TODO: Refactor

/// Structure and specification for Godwit glyphs.
#[derive(Debug, Serialize, Deserialize, StructOpt, Clone, PartialEq)]
#[serde(rename_all = "snake_case", try_from = "String", into = "String")]
pub struct Glyph {
	pub tag: String,
	pub id: String,
}

impl Default for Glyph {
	fn default() -> Self {
		Glyph {
			tag: String::default(),
			id: String::default(),
		}
	}
}

impl fmt::Display for Glyph {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "@{}/{}", &self.tag, &self.id)
	}
}

impl From<Glyph> for String {
	fn from(glyph: Glyph) -> Self {
		format!("@{}/{}", &glyph.tag, &glyph.id)
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
					id: String::from(id),
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
					id: String::from(id),
				});
			}
		}

		debug!("Attempt to parse invalid glyph {}", glyph_str);
		Err(GlyphError::InvalidGlyph {
			glyph: glyph_str.into(),
		})
	}
}
// TODO: Serializer/Deserializer

// impl Serialize for Glyph {
// 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
// 	where
// 		S: Serializer,
// 	{
// 		#[derive(Serialize)]
// 		#[serde(rename_all = "snake_case")]
// 		struct Glyph<'a> {
// 			tag: &'a String,
// 			id: i32,
// 		}
//
// 		let glyph = Glyph {
// 			tag: &self.bar,
// 			id: self.baz(),
// 		};
//
// 		Ok(serializer.serialize(serializer)?)
// 	}
// }

// impl Deserialize for Glyph {
// 	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
// 	where
// 		D: Deserializer<'a>,
// 	{
// 		let s: &str = Deserialize::deserialize(deserializer)?;
//
// 		let mut parts = s.splitn(3, " ").fuse();
//
// 		let name = parts
// 			.next()
// 			.ok_or_else(|| D::Error::custom("missing name"))?
// 			.into();
// 		let age = parts
// 			.next()
// 			.ok_or_else(|| D::Error::custom("missing age"))?
// 			.parse()
// 			.map_err(D::Error::custom)?;
// 		let alive = parts
// 			.next()
// 			.ok_or_else(|| D::Error::custom("missing alive"))?
// 			.parse()
// 			.map_err(D::Error::custom)?;
//
// 		Ok(Pet { name, age, alive })
// 	}
// }
