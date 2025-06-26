use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

// exemplos do capitulo 9 (error handling) de
// KLABNIK & NICHOLS. The Rust programming language.2021
fn _read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),        
    }
}

// igual a função anterior,mas propagando o erro com o operador ?
fn _read_username_from_file_operator() -> Result<String,io::Error>{
    let mut username_file = File::open("hello.txt")?;
    let mut username =String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// versão com implementação da standard library
// a função tenta colocar o conteúdo do path em uma string e retorna um tipo result
// se sucesso, retorna Ok(conteúdo da string)
// se fracasso, retorna Err(tipo de erro)
fn _read_username_from_file_sl() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// versão igual as anteriores, mas encadeando métodos para código menor
fn _read_username_from_file_operator_short() -> Result<String, io::Error> {
    let mut username = String::new();
    // note o operador logo após .txt") e encadeamento de método
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    // chamar o macro que provoca panic
    // panic!("crash and burn");

    // exemplo de código que panic sem provocação
    // let v =vec![1,2,3];
    // v[99];
    

    // exemplo de recovering ao abrir arquivo inexistente
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("ex_hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}",e),                    
                }                
            },
            other_error => {
                panic!("Problem opening the file : {:?}", other_error)
            }            
        }
    };
    // poderia ter comprimido tudo em 1 let
    // let greet = match File::open("hello.txt") {

       
}