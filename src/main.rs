use std::time::{SystemTime, SystemTimeError};

use yolol_devices::devices::chip::CodeRunner;
use yolol_runner::YololRunner;

fn main() -> Result<(), SystemTimeError> {
    let mut samples = vec![];
    let mut runner = YololRunner::default();
    runner.parse("speedrun.yolol");
    loop {
        let started = SystemTime::now();
        for _ in 0..1_000_000 {
            runner.step();
        }
        let lps = 1_000_000.0 / started.elapsed().unwrap().as_secs_f64();
        samples.push(lps);
        let avg = samples.iter().sum::<f64>() / samples.len() as f64;

        println!("{:.2}  l/s , {:.2} avg", lps, avg);
    }
}
