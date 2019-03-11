use std::str::FromStr;
use std::result::Result;
use std::option::Option;
use std::convert::identity;

use regex::Regex;

#[derive(Debug,Clone)]
pub enum CpuType {
    Total,
    Core(usize),
}

#[derive(Debug,Clone)]
pub struct Cpu {
    typ: CpuType,
    pub total_intr: u64,
}

impl Cpu {
    pub fn new(typ: CpuType, tail: &str) -> Option<Cpu> {
        let nums: Vec<Option<u64>> = tail
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(u64::from_str)
            .map(Result::ok)
            .collect();

        // TODO: use all of the intr data

        nums[0].map(|i| Cpu {
            typ,
            total_intr: i,
        })
    }
}

#[derive(Debug,Clone)]
pub struct Stat {
    cpu: Option<Cpu>,         // total cpu info
    cpus: Option<Vec<Cpu>>,   // specific cpu info (cpu0, cpu1, ..)
    page: Option<(u64, u64)>,  // pages paged (in, out) from disk
    swap: Option<(u64, u64)>, // pages brought (in, out)
    intr: Option<u64>,        // total num interrupts
    // TODO: disk_io
    ctxt: Option<u64>, // num context switches
    btime: Option<u64>, // time of boot up
    procs: Option<u64>, // forks since boot
    procs_running: Option<u64>,
    procs_blocked: Option<u64>,
    softirq: Option<Vec<u64>>,
}

fn to_stat(content: &str) -> Stat {
    let mut stat = Stat {
        cpu: None, cpus: None, page: None, swap: None,
        intr: None, ctxt: None, btime: None, procs: None,
        procs_running: None, procs_blocked: None, softirq: None,
    };

    let mut cpus = Vec::new();

    for line in content.lines() {
        // remove line str header
        // or continue to next line
        let tail = line.split_at(match line.find(' ') {
            Some(i) => i,
            _       => continue,
        }).1;

        let re_cpu = Regex::new(r"cpu ").unwrap();
        let re_core = Regex::new(r"cpu(?P<num>\d*) ").unwrap();

        if re_cpu.is_match(line) {
            // create cpu from string
            stat.cpu = Cpu::new(CpuType::Total, tail);

        } else if re_core.is_match(line) {
            let caps = re_core.captures(line).unwrap();

            // unwrap matched num
            let num = usize::from_str(&caps["num"]).unwrap();

            // create a cpu core from string
            let core = Cpu::new(CpuType::Core(num), tail);

            cpus.push(core);

        }
    }

    // unwrap option per numbered cpu
    let _cpus: Vec<Cpu> = cpus
        .into_iter()
        .filter_map(identity)
        .collect();

    if _cpus.len() > 0 {
        stat.cpus = Some(_cpus);
    }

    stat
}

impl Stat {
    pub fn new(stat_contents: &str) -> Stat {
        to_stat(stat_contents)
    }

    pub fn cpu(&self) -> &Option<Cpu> { &self.cpu }
}
