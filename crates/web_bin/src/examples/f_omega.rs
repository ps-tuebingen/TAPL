pub const LISTADT: &str = include_str!("../../../../examples/f_omega/listadt/listadt.omega");

pub const UNPACKLIST: &str = include_str!("../../../../examples/f_omega/unpacklist/unpacklist.omega");

pub const UNPACKPAIR: &str = include_str!("../../../../examples/f_omega/unpackpair/unpackpair.omega");

pub const PAIRADT: &str = include_str!("../../../../examples/f_omega/pairadt/pairadt.omega");

pub fn f_omega_all() -> Vec<(&'static str,&'static str)> { 
    vec![
         ("ListADT",LISTADT), 
         ("UnpackList",UNPACKLIST), 
         ("UnpackPair",UNPACKPAIR), 
         ("PairADT",PAIRADT), 

    ]
}
