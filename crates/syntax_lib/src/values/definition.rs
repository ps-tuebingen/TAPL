use crate::{
    definition::Definition,
    types::Type,
    values::{Value, ValueGroup},
};
use common::errors::ValueKind;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DefinitionValue<V, Ty>
where
    V: Value,
    Ty: Type,
{
    name: String,
    annot: Ty,
    body: V,
}

impl<V, Ty> Value for DefinitionValue<V, Ty>
where
    V: Value,
    Ty: Type,
{
    type Term = Definition<<V as Value>::Term, Ty>;

    fn knd(&self) -> ValueKind {
        self.body.knd()
    }
}

impl<V, Ty> ValueGroup for DefinitionValue<V, Ty>
where
    V: Value,
    Ty: Type,
{
    type Term = Definition<<V as Value>::Term, Ty>;
    type Type = Ty;
}

impl<V, Ty> From<DefinitionValue<V, Ty>> for Definition<<V as Value>::Term, Ty>
where
    V: Value,
    Ty: Type,
{
    fn from(def: DefinitionValue<V, Ty>) -> Definition<<V as Value>::Term, Ty> {
        Definition {
            name: def.name,
            annot: def.annot,
            body: def.body.into(),
        }
    }
}

impl<V, Ty> fmt::Display for DefinitionValue<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}:={}", self.name, self.annot, self.body)
    }
}
