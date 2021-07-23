use std::time::SystemTime;
use std::time::SystemTimeError;

use yolol_devices::devices::chip::CodeRunner;
use yolol_runner::YololRunner;

fn main() -> Result<(), SystemTimeError> {
    let mut iteration:usize = 1_000_000;
    let mut samples = vec![];
    let mut runner = YololRunner::default();
    let mut total_line_count = 0;
    runner.parse("speedrun.yolol");
    //panic!("{:?}", runner);
    loop {
        let started = SystemTime::now();
        for _ in 0..iteration {
            runner.step();
        }
        total_line_count += iteration;
        let lps = iteration as f64 / started.elapsed().unwrap().as_secs_f64();
        iteration =lps as usize / 10;
        samples.push(lps);
        let len = samples.len() as f64;
        let iter = samples.iter().rev().take(10);
        let avg = iter.clone().sum::<f64>() / iter.len() as f64;
        let sum = samples.iter().map(|i| (*i - avg).powi(2)).sum::<f64>();
        let dev = (sum / (len - 1.)).sqrt();

        println!(" - Total:\t {} lines", total_line_count);
        println!(" - Average:\t {:.2} l/s", avg);
        println!(" - StdDev:\t {:.2}", dev);

        if let Some(output) = runner.get_global().iter().find(|f| f.name() == "output") {
            if **output != "ok".into() {
                println!(
                    " - FAILED! Expected `:OUTPUT==\"ok\"`, got ``:OUTPUT==\"{}\"``",
                    **output
                );
            }
        }

        //println!("{:?}", runner.get_global().iter().find(|i| i.name() == "output"))
    }
}
