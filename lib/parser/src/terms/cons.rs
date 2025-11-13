use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::Cons};

impl<Lang> Parse for Cons<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::cons_term;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["Cons Type", "First Const Argument", "Second Cons Argument"],
        )?;

        let ty_pair = inner.remove(0);
        let ty = Lang::Type::from_pair(ty_pair, ())?;

        let fst_pair = inner.remove(0);
        let fst = Lang::Term::from_pair(fst_pair, ())?;

        let snd_pair = inner.remove(0);
        let snd = Lang::Term::from_pair(snd_pair, ())?;

        Ok(Self::new(fst, snd, ty))
    }
}
