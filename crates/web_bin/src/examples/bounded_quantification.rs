use std::collections::HashMap;
pub const SZERO: &str = include_str!("../../../../examples/bounded_quantification/szero/szero.bd");

pub const COUNTERADT: &str = include_str!("../../../../examples/bounded_quantification/counteradt/counteradt.bd");

pub const SND: &str = include_str!("../../../../examples/bounded_quantification/snd/snd.bd");

pub const PAIR: &str = include_str!("../../../../examples/bounded_quantification/pair/pair.bd");

pub const UNBOUNDID: &str = include_str!("../../../../examples/bounded_quantification/unboundid/unboundid.bd");

pub const RECORDPOLY: &str = include_str!("../../../../examples/bounded_quantification/recordpoly/recordpoly.bd");

pub const STHREE: &str = include_str!("../../../../examples/bounded_quantification/sthree/sthree.bd");

pub const FST: &str = include_str!("../../../../examples/bounded_quantification/fst/fst.bd");

pub const SPLUSPP: &str = include_str!("../../../../examples/bounded_quantification/spluspp/spluspp.bd");

pub const COUNTERTHREE: &str = include_str!("../../../../examples/bounded_quantification/counterthree/counterthree.bd");

pub const STWO: &str = include_str!("../../../../examples/bounded_quantification/stwo/stwo.bd");

pub const SSUCC: &str = include_str!("../../../../examples/bounded_quantification/ssucc/ssucc.bd");

pub const SONE: &str = include_str!("../../../../examples/bounded_quantification/sone/sone.bd");

pub fn bounded_quantification_all() -> HashMap<&'static str,&'static str> { 
    HashMap::from([
        ("SZERO",SZERO), 
("COUNTERADT",COUNTERADT), 
("SND",SND), 
("PAIR",PAIR), 
("UNBOUNDID",UNBOUNDID), 
("RECORDPOLY",RECORDPOLY), 
("STHREE",STHREE), 
("FST",FST), 
("SPLUSPP",SPLUSPP), 
("COUNTERTHREE",COUNTERTHREE), 
("STWO",STWO), 
("SSUCC",SSUCC), 
("SONE",SONE), 

    ])
}
