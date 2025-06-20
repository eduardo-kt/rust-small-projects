
fn main() {
    
    // criar vetor
    let v1 = vec![0,1,1,2,3,5,8,13];    
    let v2 = Vec::from(['a','d']);

    // vetor mutável
    let mut v3: Vec<bool> = Vec::new();

    // adicionar itens ao vetor mutável
    v3.push(true);
    v3.push(false);

    // ler elemento do vetor
    let check = &v3[1]; //panic se index fora do escopo    
    println!("{}", check);

    // ler elemento usando método .get
    let fibo = v1.get(5); // retorna None se index fora do escopo
    match fibo {
        Some(f) => println!("Sexto elemento fibonacci {}", f),
        None => println!("Não possui o sexto elemento fibonacci"),        
    }

    for letter in v2.iter() {
        for bol in v3.iter() {            
            println!("{}:= {}", letter, bol);            
        }
    }
    
}