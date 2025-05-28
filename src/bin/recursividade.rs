fn fatorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        n * fatorial(n - 1)
    }
}

fn main() {
    let valor = fatorial(5);
    println!("{}", valor);

}