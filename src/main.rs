use std::io::stdin;
use std::io::BufRead;

use yolol_devices::devices::chip::CodeRunner;
use yolol_runner::YololRunner;

fn main() {
    let mut buffer = String::new();
    let mut yolol_runner = YololRunner::default();
    yolol_runner.parse("test.yolol");
    println!("{:?}", yolol_runner);
    while stdin().lock().read_line(&mut buffer).is_ok() {
        yolol_runner.step();
        for field in yolol_runner.globals() {
            println!(":{} {}", field.name(), **field);
        }
        for field in yolol_runner.locals() {
            println!("{} {}", field.name(), **field);
        }
    }
    println!("{:?}", yolol_runner);
}
