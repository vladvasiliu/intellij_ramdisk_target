extern crate clap;

use clap::{Arg, App, crate_authors, crate_version};
use log::{info, warn, error};
use std::os::unix::fs::symlink;
use std::fs;
use std::path::PathBuf;

mod error;
mod logging;

use crate::error::{Result, Error};
use crate::logging::setup_logger;
use std::process::exit;



fn main() {
    setup_logger().unwrap();

    let mut config = Config::from_parse_args();

    let result = config.work();

    if config.should_delete_ramdir {
        match fs::remove_dir(&config.ramdir) {
            Ok(()) => warn!("Removed ramdisk directory `{}`", config.ramdir.display()),
            Err(err) => warn!("Failed to remove ramdisk directory `{}`: {}", config.ramdir.display(), err),
        }
    }

    match result {
        Ok(()) => {
            info!("Done!");
            exit(0)
        },
        Err(err) => {
            error!("Something went wrong: {}", err);
            exit(1);
        }
    }
}


struct Config {
    projdir: PathBuf,
    ramdir: PathBuf,
    should_delete_ramdir: bool,
}

impl Config {
    /// Calls parse_args and builds a Config
    pub fn from_parse_args() -> Self {
        let (projdir, ramdir) = parse_args();
        Self {
            projdir,
            ramdir,
            should_delete_ramdir: false,
        }
    }

    fn work(&mut self) -> Result<()> {
        self.create_ramdir()?;
        self.create_projdir()?;
        Ok(())
    }

    /// Creates the symlink in the project directory
    ///
    /// Returns nothing if the symlink already exists and is valid
    /// Returns an Error describing why the symlink couldn't be created
    fn create_projdir(&mut self) -> Result<()> {
        if self.projdir.is_dir() {
            warn!("Using existing project directory `{}`", self.projdir.display());
        } else {
            symlink(&self.ramdir, &self.projdir).or_else(|err| Err(Error::from_io_error(&self.projdir, err)))
                .and_then(|res| {
                    info!("Created project directory `{}`", self.projdir.display());
                    Ok(res)
                })?;
        }
        self.should_delete_ramdir = false;
        Ok(())
    }

    /// Creates the destination directory on the ramdisk
    fn create_ramdir(&mut self) -> Result<()> {
        if self.ramdir.is_dir() {
            warn!("Using existing ramdrive directory `{}`", self.ramdir.display());
            return Ok(())
        }
        fs::create_dir_all(&self.ramdir).or_else(|err| Err(Error::from_io_error(&self.ramdir, err)))
            .and_then(|res| {
                info!("Created ramdrive directory `{}`", self.ramdir.display());
                self.should_delete_ramdir = true;
                Ok(res)
            })
    }
}


fn parse_args() -> (PathBuf, PathBuf) {
    let matches = App::new("Simlink target directory.")
        .about("Sets up `target` directory as a link to a RAM disk")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("project dir")
                .short("p")
                .long("projdir")
                .value_name("PROJDIR")
                .help("Full path to the `target` directory used by the project")
                .takes_value(true)
                .required(true))
        .arg(
            Arg::with_name("ramdisk dir")
                .short("r")
                .long("ramdir")
                .value_name("RAMDIR")
                .help("Full path to the `target` directory to be created on the RAM disk")
                .required(true)
                .takes_value(true))
        .get_matches();

    (
        matches.value_of("project dir").unwrap().into(),
        matches.value_of("ramdisk dir").unwrap().into(),
    )
}

