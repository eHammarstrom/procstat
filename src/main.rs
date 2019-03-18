use std::fs::read_to_string;

mod stat;
mod safevec;
mod parse;

fn poor_mans_bench(stat_contents: &str, n: usize) {
    let mut sum: u128 = 0;
    use std::time::Instant;
    for _i in 0..n {
        let t0 = Instant::now();
        stat::Stat::new(stat_contents);
        println!("{} ns", t0.elapsed().as_nanos());
        sum += t0.elapsed().as_nanos();
    }
    println!("avg: {} ns", sum / n as u128);
}

fn main() {
    let _path = "/proc/stat";

    let stat_contents = read_to_string(_path)
        .unwrap_or_else(|_| panic!("failed to read '{}'", _path));

    poor_mans_bench(&stat_contents, 10);

    let stat = stat::Stat::new(&stat_contents);
    // println!("{}", stat_contents);
    println!("{:?}", stat);

    println!("------\n{:?}", stat.cpu);
}
