use std::thread;
use std::time::Duration;
use rand::random;
use fortytwo::number;

fn main() {
    loop {
        let n = random::<u32>();
        let m = number();
        println!("n: {} m: {}", n, m);
        thread::sleep(Duration::from_secs(1));
    }
}
