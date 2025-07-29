use derivation::ProgramDerivation;
use grammar::LanguageGrammar;
use latex::LatexFmt;
use std::fmt;
use syntax::{program::Program, terms::Term, types::Type, values::Value};
use trace::EvalTrace;

pub trait Formattable: fmt::Display + fmt::Debug + LatexFmt {}

impl<T, Ty> Formattable for Program<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
}

impl<T, Ty> Formattable for ProgramDerivation<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
}

impl<T, V> Formattable for EvalTrace<T, V>
where
    T: Term + LatexFmt,
    V: Value + LatexFmt,
{
}

impl Formattable for LanguageGrammar {}
