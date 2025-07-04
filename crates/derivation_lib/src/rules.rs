use std::fmt;

#[derive(Debug)]
pub enum TypingRule {
    App,
    Ascribe,
    Assign,
    Cast,
    Cons,
    Deref,
    Exception,
    Fix,
    False,
    Fold,
    Fst,
    Head,
    If,
    IsNil,
    IsZero,
    Lambda,
    LambdaSub,
    Left,
    Let,
    ListCase,
    Loc,
    Nil,
    Nothing,
    Num,
    Pack,
    PackBound,
    Pair,
    Pred,
    Projection,
    Raise,
    Record,
    RecordProj,
    Ref,
    Right,
    Snd,
    SomeCase,
    Something,
    Succ,
    SumCase,
    Tail,
    True,
    Try,
    TryVal,
    Tuple,
    TyApp,
    TyAppBounded,
    TyLambda,
    Unfold,
    Unit,
    Unpack,
    UnpackBounded,
    Variable,
    Variant,
    VariantCase,
    Empty,
}

impl fmt::Display for TypingRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypingRule::App => f.write_str("T-App"),
            TypingRule::Ascribe => f.write_str("T-Ascribe"),
            TypingRule::Assign => f.write_str("T-Assign"),
            TypingRule::Cast => f.write_str("T-Cast"),
            TypingRule::Cons => f.write_str("T-Cons"),
            TypingRule::Deref => f.write_str("T-Deref"),
            TypingRule::Exception => f.write_str("T-Err"),
            TypingRule::Fix => f.write_str("T-Fix"),
            TypingRule::False => f.write_str("T-False"),
            TypingRule::Fold => f.write_str("T-Fold"),
            TypingRule::Fst => f.write_str("T-Fst"),
            TypingRule::Head => f.write_str("T-Head"),
            TypingRule::If => f.write_str("T-If"),
            TypingRule::IsNil => f.write_str("T-IsNil"),
            TypingRule::IsZero => f.write_str("T-IsZero"),
            TypingRule::Lambda => f.write_str("T-Lambda"),
            TypingRule::LambdaSub => f.write_str("T-Lambda<:"),
            TypingRule::Left => f.write_str("T-Left"),
            TypingRule::Let => f.write_str("T-Let"),
            TypingRule::ListCase => f.write_str("T-ListCase"),
            TypingRule::Loc => f.write_str("T-Loc"),
            TypingRule::Nil => f.write_str("T-Nil"),
            TypingRule::Nothing => f.write_str("T-Nothing"),
            TypingRule::Num => f.write_str("T-Int"),
            TypingRule::Pack => f.write_str("T-Pack"),
            TypingRule::PackBound => f.write_str("T-Pack<:"),
            TypingRule::Pair => f.write_str("T-Pair"),
            TypingRule::Pred => f.write_str("T-Pred"),
            TypingRule::Projection => f.write_str("T-Proj"),
            TypingRule::Raise => f.write_str("T-Raise"),
            TypingRule::Record => f.write_str("T-Record"),
            TypingRule::RecordProj => f.write_str("T-RecordProj"),
            TypingRule::Ref => f.write_str("T-Ref"),
            TypingRule::Right => f.write_str("T-Right"),
            TypingRule::Snd => f.write_str("T-Snd"),
            TypingRule::SomeCase => f.write_str("T-SomeCase"),
            TypingRule::Something => f.write_str("T-Something"),
            TypingRule::Succ => f.write_str("T-Succ"),
            TypingRule::SumCase => f.write_str("T-SumCase"),
            TypingRule::Tail => f.write_str("T-Tail"),
            TypingRule::True => f.write_str("T-True"),
            TypingRule::Try => f.write_str("T-Try"),
            TypingRule::TryVal => f.write_str("T-TryV"),
            TypingRule::Tuple => f.write_str("T-Tuple"),
            TypingRule::TyApp => f.write_str("T-TyApp"),
            TypingRule::TyAppBounded => f.write_str("T-TyApp<:"),
            TypingRule::TyLambda => f.write_str("T-OpAbs"),
            TypingRule::Unfold => f.write_str("T-Unfold"),
            TypingRule::Unit => f.write_str("T-Unit"),
            TypingRule::Unpack => f.write_str("T-Unpack"),
            TypingRule::UnpackBounded => f.write_str("T-Unpack<:"),
            TypingRule::Variable => f.write_str("T-Var"),
            TypingRule::Variant => f.write_str("T-Variant"),
            TypingRule::VariantCase => f.write_str("T-VariantCase"),
            TypingRule::Empty => f.write_str(""),
        }
    }
}
