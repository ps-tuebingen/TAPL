use std::fmt;

#[derive(Debug)]
pub enum KindKind {
    Star,
    Arrow,
}

#[derive(Debug)]
pub enum TypeKind {
    Untyped,
    Variable,
    Function,
    Universal,
    Product,
    Tuple,
    Record,
    Variant,
    Sum,
    Option,
    List,
    Reference,
    Source,
    Sink,
    Existential,
    Mu,
    OpLambda,
    Nat,
    Bool,
    Unit,
    Top,
    Bot,
    OpApp,
}

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

impl fmt::Display for KindKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KindKind::Star => f.write_str("*"),
            KindKind::Arrow => f.write_str("Higher Kind"),
        }
    }
}

impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeKind::Untyped => f.write_str("untyped"),
            TypeKind::Variable => f.write_str("Type Variable"),
            TypeKind::Function => f.write_str("Function Type"),
            TypeKind::Universal => f.write_str("Universal Type"),
            TypeKind::Product => f.write_str("Product Type"),
            TypeKind::Tuple => f.write_str("Tuple Type"),
            TypeKind::Record => f.write_str("Record Type"),
            TypeKind::Variant => f.write_str("Variant Type"),
            TypeKind::Sum => f.write_str("Sum Type"),
            TypeKind::Option => f.write_str("Option Type"),
            TypeKind::List => f.write_str("List Type"),
            TypeKind::Reference => f.write_str("Reference Type"),
            TypeKind::Source => f.write_str("Source Type"),
            TypeKind::Sink => f.write_str("Sink Type"),
            TypeKind::Existential => f.write_str("Existential Type"),
            TypeKind::Mu => f.write_str("Mu Type"),
            TypeKind::OpLambda => f.write_str("Operator Abstraction"),
            TypeKind::Nat => f.write_str("Nat"),
            TypeKind::Bool => f.write_str("Bool"),
            TypeKind::Unit => f.write_str("Unit"),
            TypeKind::Top => f.write_str("Top"),
            TypeKind::Bot => f.write_str("Bot"),
            TypeKind::OpApp => f.write_str("Operator Application"),
        }
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
