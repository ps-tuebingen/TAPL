use crate::values::Value;
use std::fmt;

#[derive(Debug)]
pub enum ValueKind {
    Lambda,
    TyLambda,
    LambdaSub,
    Pair,
    Record,
    Tuple,
    Left,
    Right,
    Variant,
    Nothing,
    Something,
    Nil,
    Cons,
    Location,
    Exception,
    Fold,
    Raise,
    Package,
    True,
    False,
    Number,
    Bool,
    List,
    Option,
    Sum,
    Unit,
}

#[derive(Debug)]
pub struct ValueMismatch {
    found: ValueKind,
    expected: ValueKind,
}

impl ValueMismatch {
    pub fn new<V>(found: &V, expected: ValueKind) -> ValueMismatch
    where
        V: Value,
    {
        ValueMismatch {
            found: found.knd(),
            expected,
        }
    }
}

impl fmt::Display for ValueMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Value Mismatch:\n\tfound {},\n\texpected {}",
            self.found, self.expected
        )
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueKind::Lambda => f.write_str("Lambda Abstraction"),
            ValueKind::TyLambda => f.write_str("Type Abstraction"),
            ValueKind::LambdaSub => f.write_str("Bounded Type Abstraction"),
            ValueKind::Pair => f.write_str("Pair"),
            ValueKind::Record => f.write_str("Record"),
            ValueKind::Tuple => f.write_str("Tuple"),
            ValueKind::Left => f.write_str("Left"),
            ValueKind::Right => f.write_str("Right"),
            ValueKind::Variant => f.write_str("Variant"),
            ValueKind::Nothing => f.write_str("Nothing"),
            ValueKind::Something => f.write_str("Something"),
            ValueKind::Nil => f.write_str("Nil"),
            ValueKind::Cons => f.write_str("Cons"),
            ValueKind::Location => f.write_str("Location"),
            ValueKind::Exception => f.write_str("Exception"),
            ValueKind::Fold => f.write_str("Fold"),
            ValueKind::Raise => f.write_str("Raise"),
            ValueKind::Package => f.write_str("Package"),
            ValueKind::True => f.write_str("True"),
            ValueKind::False => f.write_str("False"),
            ValueKind::Number => f.write_str("Number"),
            ValueKind::Bool => f.write_str("Bool"),
            ValueKind::List => f.write_str("List"),
            ValueKind::Option => f.write_str("Option"),
            ValueKind::Sum => f.write_str("Sum"),
            ValueKind::Unit => f.write_str("Unit"),
        }
    }
}

impl std::error::Error for ValueMismatch {}
