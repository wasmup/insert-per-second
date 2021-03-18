// use rand::Rng;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    const N: usize = 1024;
    const MAX: usize = N * N;
    let mut m = HashMap::with_capacity(MAX);

    for _ in 0..MAX {
        let buf: Vec<u8> = (0..N).map(|_| rand::random::<u8>()).collect();
        m.insert(buf.clone(), buf);
    }
    let s = now.elapsed().as_secs();

    println!("{:?}\nlen={}, ", m.values().next(), m.len());
    println!("{}s, {} insert/s", s, MAX as f64 / s as f64); // 12s, 87381 insert/s
}
