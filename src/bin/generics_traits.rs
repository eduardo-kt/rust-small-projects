


// exemplos do capitulo 10 (generics, traits, lifetimes) de
// KLABNIK & NICHOLS. The Rust programming language.2021


// redução de código implementando funções (ex. p. 184)
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn main() {
    let nb_list1 = vec![34, 50, 25, 100, 65];
    let large1 = largest(&nb_list1);
    println!("{}", large1);

    let nb_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let large2 = largest(&nb_list2);
    println!("{}", large2);    
}