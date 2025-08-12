use crate::test::TestInclusions;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    exclude_check: bool,
    #[clap(long)]
    exclude_eval: bool,
    #[clap(long)]
    exclude_latex: bool,
}

impl Args {
    pub fn to_inclusions(&self) -> TestInclusions {
        let mut inc = TestInclusions::default();
        if self.exclude_check {
            inc.check = false;
            inc.derivation_buss = false;
            inc.derivation_frac = false;
        }

        if self.exclude_eval {
            inc.eval = false;
            inc.trace = false;
        }

        if self.exclude_latex {
            inc.derivation_buss = false;
            inc.derivation_frac = false;
            inc.grammar = false;
            inc.trace = false;
        }
        inc
    }
}
