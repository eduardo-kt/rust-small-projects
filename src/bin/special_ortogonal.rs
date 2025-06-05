use nalgebra::Matrix2;
use std::f64;

fn main() {
    let theta = 0.3;
    let so2 = x_orto(theta);
    println!("{:?}", so2);

}

fn x_orto(theta: f64) -> Matrix2<f64> {
    let sin = theta.sin();
    let cos = theta.cos();

    Matrix2::new(
        cos, -sin,
        sin, cos
    )
}
