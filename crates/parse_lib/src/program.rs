use crate::{errors::ParserError, pair_to_n_inner, Parse, Rule};
use common::errors::DuplicateDefinition;
use pest::iterators::Pair;
use syntax::{
    program::{Definition, Program},
    terms::Term,
    types::Type,
};

impl<T, Ty> Parse for Definition<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::definition;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["Definition Name", "Definition Annot", "Definition Body"],
        )?;
        let name = inner.remove(0).as_str().trim();
        let annot_rule = inner.remove(0);
        let annot = Ty::from_pair(annot_rule, ())?;
        let body_rule = inner.remove(0);
        let body = T::from_pair(body_rule, ())?;
        Ok(Definition::new(name, annot, body))
    }
}

impl<T, Ty> Parse for Program<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::program;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = p.into_inner();
        let mut prog = Program::new();
        inner.next();

        while let Some(n) = inner.next() {
            if n.as_rule() == Rule::EOI {
                return Ok(prog);
            }
            let def = Definition::<T, Ty>::from_pair(n, ())?;
            if def.name == "main" {
                if prog.main.is_none() {
                    prog.main = Some(def)
                } else {
                    return Err(DuplicateDefinition::new("main").into());
                }
            } else {
                prog.add_definition(def)?;
            }
        }

        Ok(prog)
    }
}
