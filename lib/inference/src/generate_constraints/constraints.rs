use syntax::language::Language;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Constraint<Lang>
where
    Lang: Language,
{
    Equality(EqualityConstraint<Lang>),
    Subtyping(SubtypeConstraint<Lang>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EqualityConstraint<Lang>
where
    Lang: Language,
{
    pub left: Lang::Type,
    pub right: Lang::Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubtypeConstraint<Lang>
where
    Lang: Language,
{
    pub sub_type: Lang::Type,
    pub super_type: Lang::Type,
}
