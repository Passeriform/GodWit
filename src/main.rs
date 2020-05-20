// TODO: Documentation
// TODO: Pattern matching for errors in this file
/// Utility functions and macros
pub mod _utils; // TODO: Ugly naming fix for fmt
/// Godwit Core
///
/// General abstraction over the operations used as an API with call
/// forward wrappers. It contains full API definitions and endpoints to
/// integrate Godwit core.
pub mod core;
/// Errors
///
/// Errors for Godwit suite and services.
pub mod errors;
/// Glyphs Implementation
///
/// Defines glyph as core of Godwit's datatype of choice. Implementations
/// support borrow lifecyle and can be serialized.
pub mod glyph;
/// I/O Handler
///
/// Custom wrapper handler over generic printer and scanner for terminal and HID
/// interfaces.
pub mod iohandler;
/// Plugin Interface
///
/// A composite processor for all godwit-compatible plugins.
/// Must follow a unified standard to keep minimal deviation.
pub mod plugins;
/// Godwit Settings Management
///
/// A utility abstraction over persistent settings and access methods.
pub mod settings;
/// Godwit State Handler
///
/// A core state management utility for context switching and global singletons.
pub mod statehandler;

use crate::glyph::Glyph;
use log::{debug, error, info};
use simplelog::*;
use std::path::PathBuf;
use structopt::{clap::Shell, StructOpt};

// Define CLI syntaxes.
#[derive(Debug, StructOpt)]
#[structopt(
	name = "Godwit",
	about = "An all-in-one organizational project management utility."
)]
struct CliArgs {
	/// Silence all output
	#[structopt(short, long, global = true, conflicts_with = "verbose")]
	quiet: bool,

	/// Debug mode
	#[structopt(
		short,
		long,
		global = true,
		conflicts_with = "quiet",
		parse(from_occurrences)
	)]
	verbose: u64,

	/// Organization (for all operations) (Overrides glyph)
	#[structopt(
		short,
		long = "org",
		global = true,
		// required_unless = "glyph",
		// conflicts_with = "glyph"
	)]
	organization: Option<String>,

	/// Project in organization (for all operations) (Overrides glyph)
	#[structopt(
		short,
		long,
		global = true,
		// required_unless = "glyph",
		// conflicts_with = "glyph"
	)]
	project: Option<String>,

	/// Operation selection
	#[structopt(subcommand)]
	operation: Option<OpsEnum>,
}

/// CLI call enum for Godwit core operations.
#[derive(Debug, StructOpt)]
#[structopt(about = "Operation selection")]
enum OpsEnum {
	/// Setup Godwit working directory
	Init {
		/// Target path to godwit directory (Will bind a symlink)
		target: Option<PathBuf>,

		/// Headless setup
		#[structopt(long)]
		headless: bool,

		/// Purge existing settings before setup
		#[structopt(long)]
		refresh: bool,
	},
	/// Switch to target glyph, organization or project
	Switch {
		/// Glyph (@organization/project)
		#[structopt(required_unless_all = &["project", "organization"], conflicts_with_all = &["project", "organization"])]
		glyph: Option<Glyph>,

		/// Use as default project
		#[structopt(long)]
		default: bool,
	},
	/// Add projects under Godwit
	Add {
		/// Glyph (@organization/project)
		#[structopt(required_unless_all = &["project", "organization"], conflicts_with_all = &["project", "organization"])]
		glyph: Option<Glyph>,

		/// Working path for project
		location: PathBuf,

		/// Add existing project (Doesn't trigger weaver)
		#[structopt(short, long)]
		existing: bool,

		/// Add as active project
		#[structopt(long)]
		active: bool,

		/// Add as default project
		#[structopt(long)]
		default: bool,
	},
	/// Remove projects under Godwit
	Remove {
		/// Select glyph (@organization/project)
		#[structopt(required_unless_all = &["project", "organization"], conflicts_with_all = &["project", "organization"])]
		glyph: Option<Glyph>,
	},
	/// Display Godwit's status
	Status,
}

fn get_log_level(quiet: bool, verbosity: u64) -> LevelFilter {
	if quiet {
		return LevelFilter::Off;
	} else if verbosity == 0 || verbosity == 1 {
		return LevelFilter::Warn;
	} else if verbosity == 2 {
		return LevelFilter::Info;
	} else if verbosity == 3 {
		return LevelFilter::Debug;
	} else {
		return LevelFilter::Trace;
	}
}

/// Main entry point
fn main() {
	CliArgs::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");

	let args = CliArgs::from_args();

	// Application globals
	let (organization, project) = (args.organization, args.project);
	// Logging globals
	let (verbose, verbosity, quiet) = (args.verbose > 0, args.verbose, args.quiet);

	// Logger setup
	CombinedLogger::init(vec![TermLogger::new(
		get_log_level(quiet, verbosity),
		Config::default(),
		TerminalMode::Mixed,
	)
	.unwrap()])
	.unwrap();

	match args.operation {
		Some(OpsEnum::Init {
			target,
			headless,
			refresh,
		}) => {
			debug!("Entered init operation.");
			match core::init(target, headless, refresh) {
				Ok(_) => {
					debug!("Init operation passed.");
					info!("Looks like we're good to go!");
				}
				Err(e) => {
					debug!("Init operation failed.\n{}", e);
					error!(
						"Error occured while completing setup for Godwit directories.\n{}",
						e
					);
				}
			}
		}
		Some(OpsEnum::Add {
			existing,
			glyph,
			location,
			active,
			default,
		}) => {
			debug!("Entered add operation.");

			let glyph = glyph.unwrap_or_else(|| glyph::Glyph {
				tag: organization.unwrap_or_default(),
				id: project.unwrap_or_default(),
			});

			debug!("Adding {}", &glyph);

			match core::add(glyph, location, existing, active, default) {
				Ok(_) => {
					debug!("Add operation passed.");
					// info!("Added project {}", glyph);
				}
				Err(e) => {
					debug!("Add operation failed.\n{}", e);
					error!("Error occured while adding new state.\n{}", e);
				}
			}
		}
		Some(OpsEnum::Remove { glyph }) => {
			debug!("Entered remove operation.");

			let glyph = glyph.unwrap_or_else(|| glyph::Glyph {
				tag: organization.unwrap_or_default(),
				id: project.unwrap_or_default(),
			});

			debug!("Removing {}", glyph);

			match core::remove(glyph) {
				Ok(_) => {
					debug!("Remove operation passed.");
					// info!("Removed project {}", glyph);
				}
				Err(e) => {
					debug!("Remove operation failed.\n{}", e);
					error!("Error occured while removing state.\n{}", e);
				}
			}
		}
		Some(OpsEnum::Switch { glyph, default }) => {
			debug!("Entered switch operation.");

			let glyph = glyph.unwrap_or_else(|| glyph::Glyph {
				tag: organization.unwrap_or_default(),
				id: project.unwrap_or_default(),
			});

			debug!("Switching to {}", glyph);

			match core::switch(glyph, default) {
				Ok(_) => {
					debug!("Switch operation passed.");
					// info!("Switched to project:\n{}", glyph);
				}
				Err(e) => {
					debug!("Switch operation failed.\n{}", e);
					error!("Error occured while switching projects.\n{}", e);
				}
			}
		}
		Some(OpsEnum::Status) => {
			match core::list() {
				Ok(state_list) => {
					debug!("Status operation passed.");
					iohandler::printer::print_state_graph(state_list, verbose)
						.map_err(|e| error!("{:?}", e))
						.ok();
				}
				Err(e) => {
					debug!("Status operation failed.\n{}", e);
					error!("Error occured while listing projects.\n{}", e);
				}
			};
		}
		None => {
			match core::runsplash() {
				Ok(_) => {
					debug!("Returned successfully.");
					info!("Godwit exited with success");
				}
				Err(e) => {
					debug!("An exception was thrown.\n{}", e);
					error!("Godwit threw an exception.\n{}", e);
				}
			};
		}
	}
}
