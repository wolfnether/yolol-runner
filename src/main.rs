use std::time::SystemTime;
use std::time::SystemTimeError;

use yolol_devices::devices::chip::CodeRunner;
use yolol_runner::YololRunner;

fn main() -> Result<(), SystemTimeError> {
    const LINES_PER_ITERATION: usize = 100_000_000;
    let mut samples = vec![];
    let mut runner = YololRunner::default();
    runner.parse("speedrun.yolol");
    //panic!("{:?}", runner);
    loop {
        let started = SystemTime::now();
        for _ in 0..LINES_PER_ITERATION {
            runner.step();
        }
        let lps = LINES_PER_ITERATION as f64 / started.elapsed().unwrap().as_secs_f64();
        samples.push(lps);
        let len = samples.len() as f64;
        let avg = samples.iter().sum::<f64>() / len;
        let sum = samples.iter().map(|i| (*i - avg).powi(2)).sum::<f64>();
        let dev = (sum / (len - 1.)).sqrt();

        println!(" - Total:\t {} lines", LINES_PER_ITERATION * samples.len());
        println!(" - Average:\t {:.2} l/s", avg);
        println!(" - StdDev:\t {:.2}", dev);

        //println!("{:?}", runner.get_global().iter().find(|i| i.name() == "output"))
    }
}
