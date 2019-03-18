use std::convert::identity;
use std::option::Option;
use std::str::FromStr;

use lazy_static::lazy_static;
use nom::types::CompleteStr;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::parse;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stat {
    pub cpu: Option<Cpu>,              // total cpu info
    pub cpus: Option<Vec<Cpu>>,        // specific cpu info (cpu0, cpu1, ..)
    pub page: Option<Paging>,          // in, out pages from disk
    pub swap: Option<Paging>,          // in, out swap pages
    pub intr: Option<(u64, Vec<u64>)>, // total num interrupts
    // TODO: disk_io
    pub ctxt: Option<u64>,  // num context switches
    pub btime: Option<u64>, // time of boot up
    pub procs: Option<u64>, // forks since boot
    pub procs_running: Option<u64>,
    pub procs_blocked: Option<u64>,
    pub softirq: Option<Vec<u64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Paging {
    pub _in: u64,
    pub _out: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CpuType {
    Total,
    Core(usize),
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct CpuTime {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    // Linux kernel specific
    pub iowait: Option<u64>,     // 2.5.41
    pub irq: Option<u64>,        // 2.6.0
    pub softirq: Option<u64>,    // 2.6.0
    pub steal: Option<u64>,      // 2.6.11
    pub quest: Option<u64>,      // 2.6.24
    pub quest_nice: Option<u64>, // 2.6.33
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cpu {
    pub typ: CpuType,
    pub time: CpuTime,
}

impl Cpu {
    pub fn new(typ: CpuType, tail: &str) -> Option<Cpu> {
        let res = parse::cpu_time(CompleteStr(tail));

        if let Ok((_, time)) = res {
            Some(Cpu { typ, time })
        } else {
            None
        }
    }
}

fn to_stat(content: &str) -> Stat {
    let mut stat = Stat {
        cpu: None,
        cpus: None,
        page: None,
        swap: None,
        intr: None,
        ctxt: None,
        btime: None,
        procs: None,
        procs_running: None,
        procs_blocked: None,
        softirq: None,
    };

    let mut cpus = Vec::new();

    for line in content.lines() {
        // remove line str header
        // or continue to next line
        let tail = line
            .split_at(match line.find(' ') {
                Some(i) => i,
                _ => continue,
            })
            .1;

        lazy_static! {
            static ref RE_CORE: Regex = Regex::new(r"cpu(?P<num>\d+) ").unwrap();
        }

        if RE_CORE.is_match(line) {
            // cpu# prefix parse
            let caps = RE_CORE.captures(line).unwrap();

            // unwrap matched num
            let num = usize::from_str(&caps["num"]).expect("wat");

            // create a cpu core from string
            let core = Cpu::new(CpuType::Core(num), tail);

            cpus.push(core);
        } else if &line[..3] == "cpu" {
            stat.cpu = Cpu::new(CpuType::Total, tail);
        } else if &line[..4] == "page" {
            stat.page = parse::paging(tail);
        } else if &line[..4] == "swap" {
            stat.swap = parse::paging(tail);
        } else if &line[..4] == "intr" {
            stat.intr = parse::intr(tail);
        } else if &line[..4] == "ctxt" {
            stat.ctxt = u64::from_str(tail.trim()).ok();
        } else if &line[..5] == "btime" {
            stat.btime = u64::from_str(tail.trim()).ok();
        } else if &line[..9] == "processes" {
            stat.procs = u64::from_str(tail.trim()).ok();
        } else if &line[..13] == "procs_running" {
            stat.procs_running = u64::from_str(tail.trim()).ok();
        } else if &line[..13] == "procs_blocked" {
            stat.procs_blocked = u64::from_str(tail.trim()).ok();
        } else if &line[..7] == "softirq" {
            stat.softirq = parse::str_of_nums(tail);
        }
    }

    // unwrap option per numbered cpu
    let _cpus: Vec<Cpu> = cpus.into_iter().filter_map(identity).collect();

    if !_cpus.is_empty() {
        stat.cpus = Some(_cpus);
    }

    stat
}

impl Stat {
    pub fn new(stat_contents: &str) -> Stat {
        to_stat(stat_contents)
    }
}
