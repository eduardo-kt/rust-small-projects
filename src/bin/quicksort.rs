use rand;
fn main() {
    let valores = vec![5,4,32,770,12,503,9,103,133,312];//[5,102,7,64,1,122,87,50,16];
    let resultado= quicksort(valores);
    println!("{:#?}", resultado);
    
}

fn quicksort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() < 2 {        
        return arr;
    }
    let index = rand::random_range(0..arr.len());
    let pivot = arr.remove(index);
    let mut less:Vec<i32> = Vec::new();
    let mut greater:Vec<i32> = Vec::new();
    
    for value in arr {
        if value <= pivot {
            less.push(value);
        } else {
            greater.push(value);
        }
    }
    let mut result = quicksort(less);
    result.push(pivot);
    result.extend(quicksort(greater));
    result    
}    
 