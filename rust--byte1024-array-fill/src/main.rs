use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    const N: usize = 1024;
    const MAX: usize = N * N;
    let mut m = HashMap::with_capacity(MAX);
    let mut buf = [0_u8; N];
    let mut rng = thread_rng();
    
    let now = Instant::now();
    for _ in 0..MAX {
        rng.fill(&mut buf); // array fill
        m.insert(buf.clone(), buf.clone());
    }
    let s = now.elapsed().as_secs();

    println!("{:?}\nlen={}, ", m.values().next(), m.len());
    println!("{}s, {:.0} insert/s", s, MAX as f64 / s as f64); 
}
