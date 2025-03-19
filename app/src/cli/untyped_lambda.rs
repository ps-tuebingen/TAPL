use super::{errors::Error, Source};
use untyped_lambda::{
    eval::{eval, EvalOrder as EvalOrd},
    parse::parse,
};

#[derive(clap::Args)]
pub struct Args {
    #[clap(flatten)]
    source: Source,
    /// Print additional information
    #[clap(short, long)]
    verbose: bool,
    /// Evaluation order to use when evaluating
    #[clap(flatten)]
    eo: EvalOrder,
}

#[derive(clap::Args)]
#[group(required = false, multiple = false)]
pub struct EvalOrder {
    /// Call-by-Value
    #[clap(long)]
    cbv: bool,
    /// Call-by-Name
    #[clap(long)]
    cbn: bool,
    /// Full-Beta Reduction
    #[clap(long)]
    full_beta: bool,
}

impl EvalOrder {
    fn to_lam_eval_order(&self) -> EvalOrd {
        if self.cbn {
            return EvalOrd::CBN;
        }

        if self.full_beta {
            return EvalOrd::FullBeta;
        }

        EvalOrd::CBV
    }
}

pub fn exec(args: Args) -> Result<(), Error> {
    let mut source = args.source.get_source()?;
    let parsed = parse(&mut source)?;
    if args.verbose {
        println!("parsed: {parsed}");
    }
    let evaled = eval(parsed, args.eo.to_lam_eval_order());
    println!("evaluated: {evaled}");
    Ok(())
}
