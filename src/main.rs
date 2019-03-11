#[macro_use] extern crate lazy_static;

use std::time::Instant;
use std::fs::read_to_string;

mod stat;

fn main() {
    let _path = "/proc/stat";

    let stat_contents = read_to_string(_path)
        .expect(&format!("failed to read '{}'", _path));

    for _i in 0..10 {
        let t0 = Instant::now();
        stat::Stat::new(&stat_contents);
        println!("{} ns", t0.elapsed().as_nanos());
    }

    let stat = stat::Stat::new(&stat_contents);
    println!("{:?}", stat);
}
