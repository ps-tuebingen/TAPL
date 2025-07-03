pub const COUNTERADT: &str = include_str!("../../../../examples/bounded_quantification/counteradt/counteradt.bd");

pub const RECORDPOLY: &str = include_str!("../../../../examples/bounded_quantification/recordpoly/recordpoly.bd");

pub const SPLUSPP: &str = include_str!("../../../../examples/bounded_quantification/spluspp/spluspp.bd");

pub const STHREE: &str = include_str!("../../../../examples/bounded_quantification/sthree/sthree.bd");

pub const SONE: &str = include_str!("../../../../examples/bounded_quantification/sone/sone.bd");

pub const PAIR: &str = include_str!("../../../../examples/bounded_quantification/pair/pair.bd");

pub const UNBOUNDID: &str = include_str!("../../../../examples/bounded_quantification/unboundid/unboundid.bd");

pub const SSUCC: &str = include_str!("../../../../examples/bounded_quantification/ssucc/ssucc.bd");

pub const SZERO: &str = include_str!("../../../../examples/bounded_quantification/szero/szero.bd");

pub const FST: &str = include_str!("../../../../examples/bounded_quantification/fst/fst.bd");

pub const SND: &str = include_str!("../../../../examples/bounded_quantification/snd/snd.bd");

pub const COUNTERTHREE: &str = include_str!("../../../../examples/bounded_quantification/counterthree/counterthree.bd");

pub const STWO: &str = include_str!("../../../../examples/bounded_quantification/stwo/stwo.bd");

pub fn bounded_quantification_all() -> Vec<(&'static str,&'static str)> { 
    vec![
         ("CounterADT",COUNTERADT), 
         ("PolymorphicRecord",RECORDPOLY), 
         ("SPlusPP",SPLUSPP), 
         ("SThree",STHREE), 
         ("SOne",SONE), 
         ("Pair",PAIR), 
         ("UnboundedId",UNBOUNDID), 
         ("SSucc",SSUCC), 
         ("SZero",SZERO), 
         ("Fst",FST), 
         ("Snd",SND), 
         ("CounterThree",COUNTERTHREE), 
         ("STwo",STWO), 

    ]
}
