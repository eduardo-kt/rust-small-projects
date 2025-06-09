#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn width(&self) -> bool {
        self.width > 0
    }

    fn contains(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }        
    }

    fn new(width: u32, height:u32) -> Self {
        Self { width, height }
    }
    
}

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
    
}