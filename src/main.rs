use std::env;
use std::error::Error;
use std::time::SystemTime;

use walkdir::WalkDir;
use yolol_devices::devices::chip::CodeRunner;
use yolol_runner::YololRunner;

use core_affinity;

fn main() -> Result<(), Box<dyn Error>> {
    let core_ids = core_affinity::get_core_ids().unwrap();
    core_affinity::set_for_current(core_ids[0]);
    let mut runner = YololRunner::default();

    for entry in WalkDir::new(env::current_dir()?) {
        let entry = entry?;
        if entry.file_name().to_str().map(|s| s.ends_with(".yolol")).unwrap_or(false){
            let rel_path = entry.path().strip_prefix(env::current_dir()?)?.to_str().unwrap();
            println!("## {}", rel_path);
            let timer = SystemTime::now();
            runner.parse(rel_path);
            println!(" - Compiled:\t {} ms", timer.elapsed()?.as_millis());
            
            let timer = SystemTime::now();

            let mut iteration:usize = 1_000_000;
            let mut samples = vec![];
            let mut total_line_count = 0;

            while timer.elapsed()?.as_secs_f64() < 3. {
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
            }
        }
    }
    Ok(())
}
