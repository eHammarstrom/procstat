use std::fs::read_to_string;

mod stat;

fn main() {
    let _path = "/proc/stat";

    let stat_contents = read_to_string(_path)
        .expect(&format!("failed to read '{}'", _path));

    let stat = stat::Stat::new(&stat_contents);

    println!("{:?}", stat);
}
