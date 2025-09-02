use derivations::{DefinitionDerivation, Derivation, ProgramDerivation, TypingDerivation};
use grammar::LanguageGrammar;
use latex::LatexFmt;
use std::fmt;
use syntax::{language::Language, program::Program};
use trace::EvalTrace;

pub trait Formattable: fmt::Display + fmt::Debug + LatexFmt {}

impl<Lang> Formattable for Program<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
}

impl<Lang> Formattable for ProgramDerivation<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
}

impl<Lang> Formattable for TypingDerivation<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
}

impl<Lang> Formattable for DefinitionDerivation<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
}

impl<Lang> Formattable for Derivation<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
}

impl<Lang> Formattable for EvalTrace<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Value: LatexFmt,
{
}

impl Formattable for LanguageGrammar {}
