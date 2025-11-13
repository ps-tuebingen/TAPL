use std::fmt;

#[derive(Debug)]
pub enum SubtypeRule {
    Top,
    Refl,
    Bot,
    Exists,
    All,
    Fun,
    List,
    App,
    Abs,
    AbsSub,
    Record,
    Source,
    Sink,
    Ref,
    RefSource,
    RefSink,
    Variant,
    Empty,
}

impl fmt::Display for SubtypeRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Top => f.write_str("S-Top"),
            Self::Refl => f.write_str("S-Refl"),
            Self::Bot => f.write_str("S-Bot"),
            Self::Exists => f.write_str("S-Some"),
            Self::All => f.write_str("S-All"),
            Self::Fun => f.write_str("S-Arrow"),
            Self::List => f.write_str("S-List"),
            Self::App => f.write_str("S-App"),
            Self::Abs => f.write_str("S-Abs"),
            Self::AbsSub => f.write_str("S-Abs<:"),
            Self::Record => f.write_str("S-Rcd"),
            Self::Source => f.write_str("S-Source"),
            Self::Sink => f.write_str("S-Sink"),
            Self::Ref => f.write_str("S-Ref"),
            Self::RefSource => f.write_str("S-RefSource"),
            Self::RefSink => f.write_str("S-RefSink"),
            Self::Variant => f.write_str("S-Variant"),
            Self::Empty => f.write_str(""),
        }
    }
}
