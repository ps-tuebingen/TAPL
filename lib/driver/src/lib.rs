use errors::{FileAccess, driver_error::DriverError};
use languages::dispatch::{Command, DispatchLanguage, FormatMethod, Source, create_dispatcher};

use std::{fs::File, io::Write, path::PathBuf};

pub mod cli;

use cli::Args;

pub struct Driver {
    dispatchers: Vec<Box<dyn DispatchLanguage>>,
}

impl Driver {
    pub fn new() -> Driver {
        Driver {
            dispatchers: Vec::new(),
        }
    }

    fn get_dispatcher(
        &mut self,
        lang: &str,
    ) -> Result<&mut Box<dyn DispatchLanguage>, DriverError> {
        let disp_ind = self
            .dispatchers
            .iter_mut()
            .position(|dispatcher| dispatcher.is_lang(lang));

        match disp_ind {
            Some(ind) => Ok(&mut self.dispatchers[ind]),
            None => {
                let dispatcher = create_dispatcher(lang)?;
                let last_ind = self.dispatchers.len();
                self.dispatchers.push(dispatcher);
                Ok(&mut self.dispatchers[last_ind])
            }
        }
    }

    /// Parse command line arguments and run the given command
    /// # Errors
    /// Returns an error if arguments are malformed or there is an error running the command
    pub fn run_cli(&mut self) -> Result<(), DriverError> {
        let args = <Args as clap::Parser>::parse();
        let source = args.source.get_source()?;
        let dispatcher = self.get_dispatcher(&args.lang)?;
        let res = dispatcher.run_format(source, args.cmd, args.method())?;
        args.out_file.map_or_else(
            || {
                println!("{res}");
                Ok(())
            },
            |out| self.write_to_file(&res, out),
        )
    }

    pub fn run_command(
        &mut self,
        source: Source,
        lang: &str,
        cmd: Command,
        method: FormatMethod,
    ) -> Result<String, DriverError> {
        let dispatcher = self.get_dispatcher(lang)?;
        dispatcher
            .run_format(source, cmd, method)
            .map_err(|err| err.into())
    }

    /// Write a formatted result to a given file
    /// # Errors
    /// returns an error if there is an error with file handling
    pub fn write_to_file(&self, res: &str, path: PathBuf) -> Result<(), DriverError> {
        let mut file =
            File::create(path).map_err(|err| FileAccess::new("open file for writing", err))?;
        file.write_all(res.as_bytes())
            .map_err(|err| FileAccess::new("write to file", err))?;
        Ok(())
    }
}

impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
}
