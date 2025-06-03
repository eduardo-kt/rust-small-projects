
fn main() {
    let mut lista = [7,4,6,8,102,5,9,22,66,47,200];
    for _ in lista {
        for index in 1..lista.len() {
            if lista[index-1] > lista[index] {
                let temp = lista[index-1];
                lista[index -1] = lista[index];
                lista[index] = temp;
            }
        }
    }
    println!("{:?}", lista);

}