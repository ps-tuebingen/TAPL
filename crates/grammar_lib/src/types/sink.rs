use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Sink, Type};

impl<Ty> RuleDescribe for Sink<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::call_annot("Sink", 0, Symbol::Type), "Sink Type")
    }
}
