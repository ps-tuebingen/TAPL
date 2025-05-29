pub const SZERO: &str = include_str!("../../../../examples/bounded/szero/szero.bd");

pub const COUNTERADT: &str = include_str!("../../../../examples/bounded/counteradt/counteradt.bd");

pub const SND: &str = include_str!("../../../../examples/bounded/snd/snd.bd");

pub const PAIR: &str = include_str!("../../../../examples/bounded/pair/pair.bd");

pub const UNBOUNDID: &str = include_str!("../../../../examples/bounded/unboundid/unboundid.bd");

pub const RECORDPOLY: &str = include_str!("../../../../examples/bounded/recordpoly/recordpoly.bd");

pub const STHREE: &str = include_str!("../../../../examples/bounded/sthree/sthree.bd");

pub const FST: &str = include_str!("../../../../examples/bounded/fst/fst.bd");

pub const SPLUSPP: &str = include_str!("../../../../examples/bounded/spluspp/spluspp.bd");

pub const COUNTERTHREE: &str = include_str!("../../../../examples/bounded/counterthree/counterthree.bd");

pub const STWO: &str = include_str!("../../../../examples/bounded/stwo/stwo.bd");

pub const SSUCC: &str = include_str!("../../../../examples/bounded/ssucc/ssucc.bd");

pub const SONE: &str = include_str!("../../../../examples/bounded/sone/sone.bd");

pub fn bounded_all() -> Vec<&'static str> { 
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
