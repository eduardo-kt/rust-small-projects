//! Mostra exemplos do capítulo 17 do livro 
//! Klabnik, S., & Nichols, C. (2023). The Rust programming language (2nd edition).

#[derive(Default)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        let n = self.list.len();
        self.average = total as f64 / n as f64
    }
    
}
fn main() {
    let mut vetor = AveragedCollection::default();
    vetor.add(5);
    vetor.add(4);
    vetor.add(3);
    vetor.add(2);
    vetor.add(1);
    println!("{}",vetor.average());

}
