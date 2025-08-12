pub const SZERO: &str = include_str!("../../../../examples/bounded-quantification/szero/szero.bd");

pub const COUNTERADT: &str = include_str!("../../../../examples/bounded-quantification/counteradt/counteradt.bd");

pub const SND: &str = include_str!("../../../../examples/bounded-quantification/snd/snd.bd");

pub const PAIR: &str = include_str!("../../../../examples/bounded-quantification/pair/pair.bd");

pub const UNBOUNDID: &str = include_str!("../../../../examples/bounded-quantification/unboundid/unboundid.bd");

pub const RECORDPOLY: &str = include_str!("../../../../examples/bounded-quantification/recordpoly/recordpoly.bd");

pub const STHREE: &str = include_str!("../../../../examples/bounded-quantification/sthree/sthree.bd");

pub const FST: &str = include_str!("../../../../examples/bounded-quantification/fst/fst.bd");

pub const SPLUSPP: &str = include_str!("../../../../examples/bounded-quantification/spluspp/spluspp.bd");

pub const COUNTERTHREE: &str = include_str!("../../../../examples/bounded-quantification/counterthree/counterthree.bd");

pub const STWO: &str = include_str!("../../../../examples/bounded-quantification/stwo/stwo.bd");

pub const SSUCC: &str = include_str!("../../../../examples/bounded-quantification/ssucc/ssucc.bd");

pub const SONE: &str = include_str!("../../../../examples/bounded-quantification/sone/sone.bd");

pub fn bounded-quantification_all() -> Vec<&'static str> { 
    vec![
        SZERO,
        COUNTERADT,
        SND,
        PAIR,
        UNBOUNDID,
        RECORDPOLY,
        STHREE,
        FST,
        SPLUSPP,
        COUNTERTHREE,
        STWO,
        SSUCC,
        SONE,
    ]
}
