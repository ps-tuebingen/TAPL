use std::collections::HashMap;
pub const SUMCASE: &str = include_str!("../../../../examples/inference/sumcase/sumcase.inf");

pub const DOUBLENAT: &str = include_str!("../../../../examples/inference/doublenat/doublenat.inf");

pub const SUM: &str = include_str!("../../../../examples/inference/sum/sum.inf");

pub const LIST: &str = include_str!("../../../../examples/inference/list/list.inf");

pub const NATNATNAT: &str = include_str!("../../../../examples/inference/natnatnat/natnatnat.inf");

pub const SOMETHING: &str = include_str!("../../../../examples/inference/something/something.inf");

pub const DOUBLEBOOL: &str = include_str!("../../../../examples/inference/doublebool/doublebool.inf");

pub const SOMECASE: &str = include_str!("../../../../examples/inference/somecase/somecase.inf");

pub const SWAP: &str = include_str!("../../../../examples/inference/swap/swap.inf");

pub const FIX: &str = include_str!("../../../../examples/inference/fix/fix.inf");

pub fn inference_all() -> HashMap<&'static str,&'static str> { 
    HashMap::from([
        ("SUMCASE",SUMCASE), 
("DOUBLENAT",DOUBLENAT), 
("SUM",SUM), 
("LIST",LIST), 
("NATNATNAT",NATNATNAT), 
("SOMETHING",SOMETHING), 
("DOUBLEBOOL",DOUBLEBOOL), 
("SOMECASE",SOMECASE), 
("SWAP",SWAP), 
("FIX",FIX), 

    ])
}
