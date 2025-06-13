use rand::rng;
use rand_distr::{Normal, Distribution};

fn main() {
    let time: f64 = 1.0;
    let initial = 50.0;
    let a = 20.0; // taxa de deriva
    let b = 900.0; // taxa de variância
    let steps = 10;
    let wiener_ex = wiener(initial, a, b, time, steps);
    println!("{:?}", wiener_ex);
    
}

fn wiener(initial: f64, a: f64, b: f64, time: f64, steps: usize,) -> Vec<f64> {
    let std_dev = b.sqrt();
    let dt = time / steps as f64;
    let normal = Normal::new(0.0, dt.sqrt()).unwrap();
    let mut rng = rng();

    let mut x = initial;
    let mut path = vec![x];
    for _ in 0..steps {
        let dw = normal.sample(&mut rng);
        x += a * dt + std_dev * dw;
        path.push(x);
    }
    path
}