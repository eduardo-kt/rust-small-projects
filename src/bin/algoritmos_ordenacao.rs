
fn main() {
    let mut lista = [7,4,6,8,102,5,9,22,66,47,200];
    let mut list2 = [5,8,124,1,66,12,88,3,11,55,4];
    let list3 = [32,4,106,1,99,13,27,8,44];
    let mut list4 = [108,12,3,44,1,7,99,34,8,14,32,65,20,10];
    let mut list5 = [12,2,1004,702,66,3,25,103,7];

    bubble_sort(&mut lista);
    merge_sort(&mut list4);
    println!("Merge sort: {:?}", list4);
    take_the_bigger(&mut list2);
    let sorted = selection_sort(&list3);
    println!("{:?}", sorted);

    
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

fn merge_sort(lista: &mut [i32]) {
    let len = lista.len();
    if len <= 1 {
        return;
    }
    let meio = len /2 ;
    
    let mut esquerda = lista[..meio].to_vec();
    let mut direita = lista[meio..].to_vec();
    merge_sort(&mut esquerda);
    merge_sort(&mut direita);

    let (mut i,mut j,mut k) = (0,0,0);

    while i < esquerda.len() && j < direita.len() {
        if esquerda[i] <= direita[j] {
            lista[k] = esquerda[i];
            i += 1;
        } else {
            lista[k] = direita[j];
            j += 1;
        }
        k += 1;
        }
    while i < esquerda.len() {
        lista[k] = esquerda[i];
        i += 1;
        k += 1;
    }
    while j < direita.len() {
        lista[k] = direita[j];
        j += 1;
        k += 1;
    }
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

fn _find_smaller(arr: &[i32]) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    let mut smallest_idx = 0;
    for i in 1..arr.len() {
        if arr[i] < arr[smallest_idx] {
            smallest_idx = i;
        }
    }
    Some(smallest_idx)    
}

fn selection_sort(arr: &[i32]) -> Vec<i32> {
    let mut arr_copy = arr.to_vec();
    let mut new_arr = Vec::with_capacity(arr.len());

    while !arr_copy.is_empty() {
       if let Some(smallest_idx)= _find_smaller(&arr_copy) {
           new_arr.push(arr_copy.remove(smallest_idx));
       }
    }
    new_arr
}