use clap::Parser;
use errors::driver_error::DriverError;
use languages::dispatch::{Command, FormatMethod};
use std::path::{Path, PathBuf};

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    pub lang: String,
    pub cmd: Command,
    pub out_method: Option<FormatMethod>,
    #[clap(flatten)]
    pub source: Source,
    #[clap(short, long)]
    pub out_file: Option<PathBuf>,
}

impl Args {
    #[must_use]
    pub fn method(&self) -> FormatMethod {
        self.out_method.unwrap_or(FormatMethod::Simple)
    }
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = false, multiple = false)]
pub struct Source {
    /// Load source from file
    #[clap(short, long)]
    file: Option<PathBuf>,
    /// load source from command line argument
    #[clap(short, long)]
    input: Option<String>,
}

impl Source {
    /// Get the source string specified in the command line
    /// either the passed literal string or the read file
    /// # Errors
    /// Returns an error if either no source was provided or the file could not be read
    pub fn get_source(&self) -> Result<languages::dispatch::Source, DriverError> {
        if let Some(ref src) = self.input {
            return Ok(src.as_str().into());
        }

        if let Some(ref path) = self.file {
            return Ok((path as &Path).into());
        }

        Err(DriverError::EmptyInput)
    }
}
