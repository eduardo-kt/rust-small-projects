
fn main() {
    let mut lista = [7,4,6,8,102,5,9,22,66,47,200];
    let mut list2 = [5,8,124,1,66,12,88,3,11,55,4];
    bubble_sort(&mut lista);
    merge_sort(&mut lista);
    take_the_bigger(&mut list2);

    
}

fn bubble_sort(lista: &mut [i32]) {
    let len = lista.len();
    for _ in 0..len {
        for idx in 1..len {
            if lista[idx-1] > lista[idx] {
                lista.swap(idx - 1, idx);                
            }
        }
    }
    println!("{:?}", lista);
}

fn merge_sort(_lista: &mut [i32]) {

}

fn take_the_bigger(lista: &mut [i32]) {
    let mut temp = lista.to_vec();
    let mut lista_aux = Vec::new();

    while !temp.is_empty() {
        if let Some(&maior) = temp.iter().max() {
            if let Some(idx) = temp.iter().position(|&x| x == maior) {
                temp.remove(idx);
                lista_aux.push(maior);
            }
        }
    }
    for (i, val) in lista_aux.iter().enumerate() {
        lista[i] = *val;
    }
    println!("{:?}", lista);
}