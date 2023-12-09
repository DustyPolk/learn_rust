use std::thread;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let handle = thread::spawn(|| {
        let mut counter = 0;
        while counter < 2_000_000_000 {
            counter += 1;
        }
        counter
    });

    let result = handle.join().unwrap();
    let elapsed = now.elapsed();

    println!("Counted up to: {}", result);
    println!("Time elapsed: {:?}", elapsed);
}
