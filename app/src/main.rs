use clap::Parser;
use common::language::ImplementedLanguage;
use common::{errors::Error, language::LanguageTerm};
use std::{fmt, fs::read_to_string, path::PathBuf};

fn run_lang(lang: &ImplementedLanguage, src: String, debug: bool) -> Result<(), Error> {
    match lang {
        ImplementedLanguage::UntypedArithmetic => {
            run::<untyped_arithmetic::terms::Term>(src, debug)
        }
        ImplementedLanguage::UntypedLambda => run::<untyped_lambda::terms::Term>(src, debug),
        ImplementedLanguage::TypedArithmetic => run::<typed_arithmetic::terms::Term>(src, debug),
        ImplementedLanguage::Stlc => run::<stlc::terms::Term>(src, debug),
        ImplementedLanguage::References => run::<references::terms::Term>(src, debug),
        ImplementedLanguage::Exceptions => run::<exceptions::terms::Term>(src, debug),
        ImplementedLanguage::Subtypes => run::<subtypes::terms::Term>(src, debug),
        ImplementedLanguage::Recursive => run::<recursive::terms::Term>(src, debug),
        ImplementedLanguage::Existential => run::<existential::terms::Term>(src, debug),
        ImplementedLanguage::SystemF => run::<system_f::terms::Term>(src, debug),
        ImplementedLanguage::BoundedQuantification => {
            run::<bounded_quantification::terms::Term>(src, debug)
        }
        ImplementedLanguage::LambdaOmega => run::<lambda_omega::terms::Term>(src, debug),
        ImplementedLanguage::FOmega => run::<f_omega::terms::Term>(src, debug),
        ImplementedLanguage::FOmegaSub => run::<f_omega_sub::terms::Term>(src, debug),
    }
}

#[derive(Parser)]
pub struct Args {
    lang: ImplementedLanguage,
    #[clap(flatten)]
    source: Source,
    #[clap(short, long)]
    verbose: bool,
    #[clap(short, long)]
    debug: bool,
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = true, multiple = false)]
pub struct Source {
    /// Load source from file
    #[clap(short, long)]
    file: Option<PathBuf>,
    /// load source from command line argument
    #[clap(short, long)]
    input: Option<String>,
}

impl Source {
    fn get_source(&self) -> String {
        if let Some(ref src) = self.input {
            return src.clone();
        }

        if let Some(ref path) = self.file {
            let contents = read_to_string(path).expect("Could not read input file");
            return contents;
        }

        panic!("Either --file or --input must be provided")
    }
}

pub fn run<T>(src: String, debug: bool) -> Result<(), Error>
where
    T: LanguageTerm,
{
    let parsed = T::parse(src)?;
    display_or_debug(&parsed, debug);
    let checked = parsed.check_start()?;
    display_or_debug(&checked, debug);
    let evaled = parsed.eval_start()?;
    println!("evaled: {}", evaled);
    Ok(())
}

pub fn display_or_debug<T: fmt::Debug + fmt::Display>(t: &T, debug: bool) -> String {
    if debug {
        format!("{t:?}")
    } else {
        format!("{t}")
    }
}

fn main() {
    let args = Args::parse();
    let src = args.source.get_source();
    match run_lang(&args.lang, src, args.debug) {
        Ok(()) => (),
        Err(err) => println!("Program exited with error:\n{}", err),
    }
}
