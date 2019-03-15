use std::str::FromStr;
use nom::{
    types::CompleteStr,
    do_parse,opt,named,map_res,
    space,digit
};

use crate::stat::CpuTime;

// digit to u64 parser
named!(_u64<CompleteStr, u64>,
       map_res!(digit, |CompleteStr(s)| u64::from_str(s))
);

named!(pub cpu_time<CompleteStr, CpuTime>, do_parse!(
    space >>
    user: _u64 >> space >>
    nice: _u64 >> space >>
    system: _u64 >> space >>
    idle: _u64 >> space >>
    iowait: opt!(do_parse!(v: _u64 >> space >> (v))) >>
    irq: opt!(do_parse!(v: _u64 >> space >> (v))) >>
    softirq: opt!(do_parse!(v: _u64 >> space >> (v))) >>
    steal: opt!(do_parse!(v: _u64 >> space >> (v))) >>
    quest: opt!(do_parse!(v: _u64 >> space >> (v))) >>
    quest_nice: opt!(do_parse!(v: _u64 >> (v))) >>
    (CpuTime {
        user, nice, system, idle,
        iowait, irq, softirq, steal,
        quest, quest_nice
    })
));