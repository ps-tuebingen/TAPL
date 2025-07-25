pub const RESETCOUNTERCLASS: &str = include_str!("../../../../examples/f_omega_sub/resetcounterclass/resetcounterclass.omega");

pub const SENDINC: &str = include_str!("../../../../examples/f_omega_sub/sendinc/sendinc.omega");

pub const NEWCOUNTER: &str = include_str!("../../../../examples/f_omega_sub/newcounter/newcounter.omega");

pub const SENDGET: &str = include_str!("../../../../examples/f_omega_sub/sendget/sendget.omega");

pub const SENDRESET: &str = include_str!("../../../../examples/f_omega_sub/sendreset/sendreset.omega");

pub const COUNTERCLASS: &str = include_str!("../../../../examples/f_omega_sub/counterclass/counterclass.omega");

pub fn f_omega_sub_all() -> Vec<(&'static str,&'static str)> { 
    vec![
         ("ResetCounterClass",RESETCOUNTERCLASS), 
         ("SendInc",SENDINC), 
         ("NewCounter",NEWCOUNTER), 
         ("SendGet",SENDGET), 
         ("SendReset",SENDRESET), 
         ("CounterClass",COUNTERCLASS), 

    ]
}
