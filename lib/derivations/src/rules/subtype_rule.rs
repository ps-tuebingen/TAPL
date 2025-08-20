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
}

impl fmt::Display for SubtypeRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubtypeRule::Top => f.write_str("S-Top"),
            SubtypeRule::Refl => f.write_str("S-Refl"),
            SubtypeRule::Bot => f.write_str("S-Bot"),
            SubtypeRule::Exists => f.write_str("S-Some"),
            SubtypeRule::All => f.write_str("S-All"),
            SubtypeRule::Fun => f.write_str("S-Arrow"),
            SubtypeRule::List => f.write_str("S-List"),
            SubtypeRule::App => f.write_str("S-App"),
            SubtypeRule::Abs => f.write_str("S-Abs"),
            SubtypeRule::AbsSub => f.write_str("S-Abs<:"),
            SubtypeRule::Record => f.write_str("S-Rcd"),
            SubtypeRule::Source => f.write_str("S-Source"),
            SubtypeRule::Sink => f.write_str("S-Sink"),
            SubtypeRule::Ref => f.write_str("S-Ref"),
            SubtypeRule::RefSource => f.write_str("S-RefSource"),
            SubtypeRule::RefSink => f.write_str("S-RefSink"),
            SubtypeRule::Variant => f.write_str("S-Variant"),
        }
    }
}
