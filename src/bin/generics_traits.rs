


// exemplos do capitulo 10 (generics, traits, lifetimes) de
// KLABNIK & NICHOLS. The Rust programming language.2021


// redução de código implementando funções (ex. p. 184)
// modifica função para tipo genérico
// implementa trait PartialOrd para lidar cokm comparações
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T, Q> {
    x: T,
    y: Q,
}



fn main() {
    let list1 = vec![34, 50, 25, 100, 65];
    let large1 = largest(&list1);
    println!("{}", large1);

    let list2 = vec!['h', 'a', 'r', 'q', 'z', '2', '4', 'p'];
    let large2 = largest(&list2);
    println!("{}", large2);    

    let ponto = Point{x:4.0, y:5};
    println!("{} == {}", ponto.x, ponto.y);
}