fn main() {
    let vetor = vec!(1,7,2,9,3,6,4,12);
    let filtrado: Vec<i32>= vetor.clone().into_iter().filter(|x| *x > 5).collect();
    println!("Vetor completo {:?}", vetor);
    println!("Vetor filtrado {:?}", filtrado);
}