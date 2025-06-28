#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.height * self.width
    }
    pub fn width(&self) -> bool {
        self.width > 0
    }

    pub fn contains(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }        
    }

    pub fn new(width: u32, height:u32) -> Self {
        Self { width, height }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_contains_smaller() {
        let larger = Rectangle::new(23, 12);
        let smaller = Rectangle::new(7, 10);
        assert!(larger.contains(&smaller));
    }
        #[test]
    fn smaller_cannot_contains_larger() {
        let larger = Rectangle::new(30, 15);
        let smaller = Rectangle::new(7, 25);
        assert!(!smaller.contains(&larger));
    }
}
