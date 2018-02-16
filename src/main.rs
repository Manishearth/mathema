#![allow(dead_code)]
#![deny(unused_must_use)] // always a bug
#![feature(crate_in_paths, conservative_impl_trait, crate_visibility_modifier, decl_macro,
           dyn_trait, /*FIXME(rust-lang/rust#47075) extern_absolute_paths,*/
           in_band_lifetimes, match_default_bindings, nll,
           termination_trait, underscore_lifetimes, universal_impl_trait)]

// FIXME can't use this because of format!
//#![deny(elided_lifetime_in_path)]

// FIXME rust-lang/rust#47075
#[cfg(test)]
extern crate assert_cli;
extern crate atomicwrites;
extern crate chrono;
extern crate failure;
extern crate git2;
extern crate itertools;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate structopt_derive;
extern crate structopt;
#[cfg(test)]
extern crate tempdir;
extern crate uuid;
extern crate walkdir;

use structopt_derive::StructOpt;
use structopt::StructOpt;
use crate::prelude::*;

macro throw($t:expr) {
    return Err($t.into());
}

mod add;
mod cards;
mod db;
mod errors;
mod git;
mod line_parser;
mod new;
mod status;
mod prelude;
mod test;
mod uuid_ext;

/// Do fancy things
#[derive(StructOpt, Debug)]
#[structopt(name = "mathema", about = "a CLI for flashcards")]
struct Mathema {
    #[structopt(name = "directory", help = "where your existing cards can be found")]
    directory: Option<String>,

    #[structopt(subcommand)]
    command: MathemaCommand,
}

#[derive(StructOpt, Debug)]
enum MathemaCommand {
    #[structopt(name = "quiz", about = "test yourself")] Quiz,

    #[structopt(name = "dump", about = "dump info about cards")] Dump,

    #[structopt(name = "new", about = "create a new deck of cards")]
    New {
        #[structopt(help = "where to create your cards")]
        directory: String,
    },

    #[structopt(name = "status", about = "check on the status of your cards")]
    Status,

    #[structopt(name = "add", about = "add new cards from file")]
    Add {
        #[structopt(help = "new card file")]
        file: String,

        #[structopt(short = "f", long = "force", help = "continue despite ignorable errors")]
        force: bool,
    },
}

fn main() {
    match main1() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err);
            ::std::process::exit(1);
        }
    }
}

fn main1() -> Result<(), Error> {
    let args = Mathema::from_args();

    let existing_directory = &match args.directory {
        Some(s) => Path::new(&s).to_owned(),
        None => env::current_dir()?,
    };

    match args.command {
        MathemaCommand::Quiz => {
            println!("Don't you feel smarter?");
        }

        MathemaCommand::New { directory } => {
            new::new(directory)?;
        }

        MathemaCommand::Status => {
            status::status(&existing_directory)?;
        }

        MathemaCommand::Add { file, force } => {
            add::add(&existing_directory, file, force)?;
        }

        MathemaCommand::Dump {} => {
            println!("Dumping cards");
        }
    }
    Ok(())
}
