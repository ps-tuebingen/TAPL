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

pub fn inference_all() -> Vec<(&'static str,&'static str)> { 
    vec![
        ("]]]l[]w",SUMCASE), 
("o]]:]w][]",DOUBLENAT), 
("]]]",SUM), 
("]]]]",LIST), 
("][]][]][]",NATNATNAT), 
("]]]w]]]]r",SOMETHING), 
("o]]:]w:]]]",DOUBLEBOOL), 
("]]]wl[]w",SOMECASE), 
("]][]",SWAP), 
("e]]",FIX), 

    ]
}
