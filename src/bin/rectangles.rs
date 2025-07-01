use rust_small_projs::rectangles::Rectangle;

fn main() {
    let ret1 = Rectangle {
        width: 30,
        height: 23,
    };
    let ret2 = Rectangle {
        width: 12,
        height: 7,
    };
    let sqr = Rectangle::square(8);

    let ret3 = Rectangle::new(7, 11);

    println!("{:#?} {} {} {}", ret1, ret2.width(), ret1.area(), ret1.contains(&ret2));
    println!("{:#?}", sqr);
    println!("{:#?}", ret3.area());

    let mut list = [
        Rectangle::new(32, 7),
        Rectangle::new(12, 22),
        Rectangle::new(7, 20),
        Rectangle::new(16, 12),
    ];
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
    
}