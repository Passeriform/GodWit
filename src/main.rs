/// Main Handler
///
/// Godwit core wrapper for utility exposure. Used as an example of godwit API and abstraction usage.
///
/// Structs/Enums:
///    CliArgs -> Defines CLI syntaxes using StructOpt.
///    OpsEnum -> CLI call enum for Godwit core operations.

/// Implementations:
///    None

/// Functions:
///    main -> Entry point/cli parser

use structopt::StructOpt;
use std::path::PathBuf;
use env_logger;
use log::{
    info,
    error,
    debug
};

pub mod core;
pub mod glyph;
pub mod iohandler;
pub mod statehandler;
pub mod plugins;
pub mod settings;

#[derive(Debug, StructOpt)]
#[structopt(name = "Godwit", about = "An all-in-one organizational project management utility.")]
struct CliArgs {
    /// Activate debug mode
    #[structopt(short, long)]
    debug:              bool,

    /// Select organization name (optional)
    #[structopt(short, long = "org")]
    organization:   Option<String>,

    /// Select project name (optional)
    #[structopt(short, long)]
    project:        Option<String>,

    /// Select an operation
    #[structopt(subcommand)]
    operation:      Option<OpsEnum>,
}


#[derive(Debug, StructOpt)]
#[structopt(about = "Operation selection")]
enum OpsEnum {
    /// Setup Godwit for the first time
    Init {
        /// Give a target path to setup
        target:         Option<PathBuf>,

        /// Configure a headless setup
        #[structopt(long)]
        headless:       Option<bool>,
    },
    /// Switch to target glyph, organization or project
    Switch {
        /// Select oranization
        #[structopt(short, long = "org", requires = "project", required_unless = "glyph", conflicts_with = "glyph")]
        organization:   Option<String>,

        /// Select project in current organization
        #[structopt(short, long, required_unless = "glyph", conflicts_with = "glyph")]
        project:        Option<String>,

        /// Select glyph (@organization/project)
        #[structopt(required_unless_all = &["project", "organization"], conflicts_with_all = &["project", "organization"])]
        glyph:          Option<String>,
    },
    /// Add projects under Godwit
    Add {
        /// Choose if project is pre-existing
        #[structopt(short, long)]
        existing:        Option<bool>,

        /// Select glyph (@organization/project)
        #[structopt(required_unless_all = &["project", "organization"], conflicts_with_all = &["project", "organization"])]
        glyph:          Option<String>,

        /// Select oranization
        #[structopt(short, long = "org", requires = "project", required_unless = "glyph", conflicts_with = "glyph")]
        organization:   Option<String>,

        /// Select project in current organization
        #[structopt(short, long, required_unless = "glyph", conflicts_with = "glyph")]
        project:        Option<String>,

        /// Select working location for project
        location:       PathBuf,
    },
    /// List all added projects
    Status {
        /// Choose if project is pre-existing
        #[structopt(short, long="verbose")]
        verbose:        bool,
    },
    /// Kill the current save state
    KillSave,
}

fn main() {
    env_logger::init();
    let _args = CliArgs::from_args();

    match _args.operation {
        Some(OpsEnum::Init { target, headless }) => {
            debug!("[core] Entered init operation.");
            match core::init(target, headless) {
                Ok(_) => {
                    debug!("[init] Init operation passed.");
                    info!("Looks like we're good to go!");
                }
                Err(e) => {
                    debug!("[init] Init operation failed.\n{}", e);
                    error!("Error occured while completing setup for Godwit directories.\n{}", e);
                }
            }
        }
        Some(OpsEnum::Add { existing, glyph, organization, project, location }) => {
            debug!("[core] Entered add operation.");

            let glyph_str: String = glyph.unwrap_or_else(||
                glyph::new(
                    organization.unwrap().as_str(),
                    project.unwrap().as_str()
                ).to_str()
            );

            debug!("[add] Adding {}", glyph_str);

            match core::add(
                glyph::from(glyph_str)
                .unwrap_or(glyph::Glyph {..Default::default()}),
                location,
                existing
            ) {
                Ok(glyph) => {
                    debug!("[add] Add operation passed.");
                    info!("Added to project:\n{:#?}", glyph);
                },
                Err(e) => {
                    debug!("[add] Add operation failed.\n{}", e);
                    error!("Error occured while adding new state.\n{}", e);
                }
            }
        }
        Some(OpsEnum::Switch { glyph, organization, project }) => {
            debug!("[core] Entered switch operation.");

            let glyph_str: String = glyph.unwrap_or(
                glyph::new(
                    organization.unwrap().as_str(),
                    project.unwrap().as_str()
                ).to_str()
            );

            debug!("[switch] Switching to {}", glyph_str);

            match core::switch(
                glyph::from(glyph_str)
                .unwrap_or(glyph::Glyph {..Default::default()})
            ) {
                Ok(glyph) => {
                    debug!("[switch] Switch operation passed.");
                    info!("Switched to project:\n{:#?}", glyph);
                },
                Err(e) => {
                    debug!("[switch] Switch operation failed.\n{}", e);
                    error!("Error occured while switching projects.\n{}", e);
                }
            }
        }
        Some(OpsEnum::Status { verbose }) => {
            match core::list() {
                Ok(project_list) => {
                    debug!("[status] List operation passed.");
                    iohandler::printer::print_state_graph(project_list, Some(verbose));
                },
                Err(e) => {
                    debug!("[status] Switch operation failed.\n{}", e);
                    error!("Error occured while listing projects.\n{}", e);
                }
            };
        }
        Some(OpsEnum::KillSave) => {
            core::killsave();
        }
        None => {
            match core::runsplash() {
                Ok(_) => {
                    debug!("[tui] Returned successfully.");
                    info!("Godwit exited with success");
                },
                Err(e) => {
                    debug!("[tui] An exception was thrown.\n{}", e);
                    error!("Godwit threw an exception.\n{}", e);
                }
            };
        }
    }
}
