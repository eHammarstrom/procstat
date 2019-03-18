#![feature(test)]

extern crate test;

use std::fs::read_to_string;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};

use serde_json;

mod parse;
mod safevec;
mod stat;

fn parse_forever(stat_lock: Arc<RwLock<stat::Stat>>, path: &str) {
    println!("Parsing {} path every 1ms.", path);
    // compensate for wake up latency of thread
    let mut sleep_compensation: u32 = 0;

    loop {
        // TODO: timestamp before and after sleep to calc avg. latency
        thread::sleep(Duration::new(0, 1000000 - sleep_compensation));

        // read and parse stat file
        let stat_contents =
            read_to_string(path).unwrap_or_else(|_| panic!("failed to read '{}'", path));
        let stat = stat::Stat::new(&stat_contents);

        // write new stat state to shared memory
        {
            let mut s = stat_lock.write().unwrap();
            *s = stat;
        }
    }
}

struct Server {
    out: Sender,
    stat_lock: Arc<RwLock<stat::Stat>>,
    freq: Duration,
    alive: Arc<RwLock<bool>>,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // start a msg thread that outputs stat at freq
        let stat_lock = self.stat_lock.clone();
        let alive = self.alive.clone();
        let freq = self.freq.clone();
        let out = self.out.clone();

        thread::spawn(move || {
            while *alive.read().unwrap() {
                thread::sleep(freq);

                let stat = stat_lock.read().unwrap();
                // convert stat to json string
                let stat_to_json = serde_json::to_string(&(*stat));
                let status = match stat_to_json {
                    Ok(stat_json) => out.send(stat_json),
                    Err(e) => {
                        println!("Thread died with, {}", e);
                        *alive.write().unwrap() = false;
                        return;
                    }
                };

                if status.is_err() {
                    *alive.write().unwrap() = false;
                    return;
                }
            }
        });

        Ok(())
    }

    // TODO: allow adjustment of send frequency
    fn on_message(&mut self, msg: Message) -> Result<()> {
        Ok(println!("\"{}\" said Roger.", msg))
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Jim, he's dead.");
        // kill worker thread
        *self.alive.write().unwrap() = false;
    }
}

fn main() {
    let _path = "/proc/stat";
    let stat_contents =
        read_to_string(_path).unwrap_or_else(|_| panic!("failed to read '{}'", _path));
    let stat = stat::Stat::new(&stat_contents);

    // initial lock state
    let stat_lock = Arc::new(RwLock::new(stat));
    let ws_stat_lock = stat_lock.clone();

    // start read, parse thread
    thread::spawn(move || parse_forever(stat_lock.clone(), _path));

    listen("127.0.0.1:3012", |out| Server {
        out,
        stat_lock: ws_stat_lock.clone(),
        freq: Duration::new(1, 0),
        alive: Arc::new(RwLock::new(true)),
    })
    .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::*;
    use test::Bencher;

    #[bench]
    fn stat_read_and_parse(b: &mut Bencher) {
        b.iter(|| {
            let _path = "/proc/stat";
            let stat_contents =
                read_to_string(_path).unwrap_or_else(|_| panic!("failed to read '{}'", _path));
            stat::Stat::new(&stat_contents);
        })
    }

    #[bench]
    fn stat_parse(b: &mut Bencher) {
        let _path = "/proc/stat";
        let stat_contents =
            read_to_string(_path).unwrap_or_else(|_| panic!("failed to read '{}'", _path));

        b.iter(|| stat::Stat::new(&stat_contents))
    }
}
