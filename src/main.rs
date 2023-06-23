use std::thread;
use std::time::Duration;
use rand::random;

fn main() {
    loop {
        let n = random::<u32>();
        println!("n: {}", n);
        thread::sleep(Duration::from_secs(1));
    }
}
