use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    const N: usize = 1024;
    const MAX: usize = N * N;
    let mut m = HashMap::with_capacity(MAX);

    let now = Instant::now();
    for _ in 0..MAX {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(N)
            .map(char::from)
            .collect();

        m.insert(s.clone(), s);
    }
    let s = now.elapsed().as_secs();

    println!("{:?}\nlen={}, ", m.values().next(), m.len());
    println!("{}s, {:.0} insert/s", s, MAX as f64 / s as f64); 
}
