use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    const N: usize = 1024;
    const MAX: usize = N * N;
    let mut m = HashMap::with_capacity(MAX);
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
    let between = Uniform::from(0..letters.len());
    let mut buf = [0_u8; N];

    let now = Instant::now();
    for _ in 0..MAX {
        let mut rng = rand::thread_rng();
        for i in 0..N {
            buf[i] = letters[between.sample(&mut rng)];
        }
        m.insert(buf.clone(), buf.clone());
    }
    let s = now.elapsed().as_secs();

    println!("{:?}\nlen={}, ", m.values().next(), m.len());
    println!("{}s, {:.0} insert/s", s, MAX as f64 / s as f64); // 9s, 116_508 insert/s
}
