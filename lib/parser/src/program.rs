use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::{MissingInput, UndefinedMain, UnexpectedRule, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::{definition::Definition, program::Program, terms::Term, types::Type};

impl<T, Ty> Parse for Program<T, Ty>
where
    T: Term + GroupParse,
    Ty: Type + GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::program;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, ParserError> {
        let inner = p.into_inner();
        let mut defs = vec![];
        let mut main = None;

        for n in inner {
            if n.as_rule() == Rule::EOI {
                if let Some(mn) = main {
                    return Ok(Program::new(mn, defs));
                } else {
                    return Err(UndefinedMain.into());
                }
            };
            let def_rule = pair_to_n_inner(n, vec!["Definition"])?.remove(0);
            match def_rule.as_rule() {
                Rule::top_level_def => {
                    let def = Definition::<T, Ty>::from_pair(def_rule, ())?;
                    defs.push(def);
                }
                Rule::main_def => {
                    let term_rule = pair_to_n_inner(def_rule, vec!["Main Body"])?.remove(0);
                    let main_body = T::from_pair(term_rule, ())?;
                    main = Some(main_body);
                }

                _ => {
                    return Err(UnexpectedRule::new(
                        &format!("{:?}", def_rule.as_rule()),
                        "Definition",
                    )
                    .into());
                }
            }
        }

        Err(MissingInput::new("EOI").into())
    }
}
