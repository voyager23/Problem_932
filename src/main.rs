fn main() {
    for n in 1..26 {
        let m = 10000 - 396 * n;
        let p: f64 = (m as f64).sqrt();
        let q: f64 = p.floor();
        if p - q == 0.0 {
            println!("n={0} : determinant = {1} ", n, m);
        }
    }
}

