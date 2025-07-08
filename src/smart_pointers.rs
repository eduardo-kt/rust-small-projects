//! Implementa exemplos do capítulo 15 de
//! Klabnik, S., & Nichols, C. (2023). The Rust programming language
use std::ops::Deref;

pub struct MyBox<T>(T);


impl <T> MyBox<T> {
    pub fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }    
}

impl<T> Deref for MyBox<T>  {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
    
}