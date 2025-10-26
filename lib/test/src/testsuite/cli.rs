use crate::config::{KEY_BUSS, KEY_CHECK, KEY_EVAL, KEY_FRAC, KEY_REPARSE, KEY_TRACE, TestConfig};
use clap::Parser;

/// Command line arguments for the test suite
/// used to disable certain tests
/// in particular, excluding latex speeds up tests significantly
#[derive(Parser)]
pub struct Args {
    /// Exclude type checking
    #[clap(long)]
    pub exclude_check: bool,
    #[clap(long)]
    /// Exclude evaluation
    pub exclude_eval: bool,
    #[clap(long)]
    /// Exclude latex compiling
    /// this includes derivations (buss/frac+array) and traces
    pub exclude_latex: bool,
    #[clap(long)]
    /// Exclude reparsing
    pub exclude_reparse: bool,
}

impl Args {
    /// update a [`TestConfig`] (in particular the `exclusions` field) with given cli arguments
    pub fn update_conf(&self, conf: &mut TestConfig) {
        if self.exclude_check {
            conf.exclusions.insert(KEY_CHECK.to_owned(), true);
        }
        if self.exclude_eval {
            conf.exclusions.insert(KEY_EVAL.to_owned(), true);
        }
        if self.exclude_latex {
            conf.exclusions.insert(KEY_BUSS.to_owned(), true);
            conf.exclusions.insert(KEY_FRAC.to_owned(), true);
            conf.exclusions.insert(KEY_TRACE.to_owned(), true);
        }
        if self.exclude_reparse {
            conf.exclusions.insert(KEY_REPARSE.to_owned(), true);
        }
    }
}
