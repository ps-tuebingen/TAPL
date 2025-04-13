use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Lang {
    BoundedQuantification,
    Exceptions,
    Existential,
    Featherweight,
    FOmega,
    FOmegaSub,
    Inference,
    LambdaOmega,
    Recursive,
    References,
    Stlc,
    Subtypes,
    SystemF,
    TypedArithmetic,
    UntypedArithmetic,
    UntypedLambda,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lang::BoundedQuantification => f.write_str("Bounded Quantification"),
            Lang::Exceptions => f.write_str("Exceptions"),
            Lang::Existential => f.write_str("Existential"),
            Lang::Featherweight => f.write_str("Featherweight"),
            Lang::FOmega => f.write_str("FOmega"),
            Lang::FOmegaSub => f.write_str("FOmegaSub"),
            Lang::Inference => f.write_str("Inference"),
            Lang::LambdaOmega => f.write_str("LambdaOmega"),
            Lang::Recursive => f.write_str("Recursive"),
            Lang::References => f.write_str("References"),
            Lang::Stlc => f.write_str("STLC"),
            Lang::Subtypes => f.write_str("Subtypes"),
            Lang::SystemF => f.write_str("SystemF"),
            Lang::TypedArithmetic => f.write_str("TypedArithmetic"),
            Lang::UntypedArithmetic => f.write_str("UntypedArithmetic"),
            Lang::UntypedLambda => f.write_str("UntypedLabmda"),
        }
    }
}
