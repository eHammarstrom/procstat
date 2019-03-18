use std::str::FromStr;
use std::convert::identity;

use nom::{
    types::CompleteStr,
    do_parse,opt,named,map_res,
    space,digit
};

use crate::stat::CpuTime;
use crate::stat::Paging;

// digit to u64 parser
named!(_u64<CompleteStr, u64>, map_res!(digit, |CompleteStr(s)| u64::from_str(s)));

named!(pub cpu_time<CompleteStr, CpuTime>, do_parse!(
    space >>
    user: _u64 >> space >>
    nice: _u64 >> space >>
    system: _u64 >> space >>
    idle: _u64 >> space >>
        // >= Linux 2.5.41
    iowait: opt!(do_parse!(v: _u64 >> space >> (v))) >>
        // >= Linux 2.6.0
    irq: opt!(do_parse!(v: _u64 >> space >> (v))) >>
    softirq: opt!(do_parse!(v: _u64 >> space >> (v))) >>
        // >= Linux 2.6.11
    steal: opt!(do_parse!(v: _u64 >> space >> (v))) >>
        // >= Linux 2.6.24
    quest: opt!(do_parse!(v: _u64 >> space >> (v))) >>
        // >= Linux 2.6.33
    quest_nice: opt!(do_parse!(v: _u64 >> (v))) >>
    (CpuTime {
        user, nice, system, idle,
        iowait, irq, softirq, steal,
        quest, quest_nice
    })
));

pub fn str_of_nums(tail: &str) -> Option<Vec<u64>> {
    let parsed_nums: Vec<Option<u64>> = tail
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(u64::from_str)
        .map(Result::ok)
        .collect();

    let nums: Vec<u64> = parsed_nums
        .into_iter()
        .filter_map(identity)
        .collect();

    if nums.is_empty() {
        None
    } else {
        Some(nums)
    }
}

pub fn intr(tail: &str) -> Option<(u64, Vec<u64>)> {
    let opt_nums = str_of_nums(tail);

    opt_nums.map(|mut nums| {
        // parse_nums returns vec.len() >= 1
        let head = nums.remove(0);
        (head, nums)
    })
}

pub fn paging(tail: &str) -> Option<Paging> {
    let opt_nums = str_of_nums(tail);

    opt_nums.map(|nums| {
        Paging {
            _in: nums[0],
            _out: nums[1],
        }
    })
}
