use std::cmp::Ordering;


fn busca_sequencial(lista: &[i32], alvo: i32) -> Option<usize> {
    for (indice, &valor) in lista.iter().enumerate() {
        if valor == alvo {
            return Some(indice)
        }
    }
    None
}

fn busca_binaria(mut lista: Vec<i32>, alvo: i32) -> Option<usize> {
    lista.sort();
    let mut inicio = 0;
    let mut fim = lista.len().saturating_sub(1);

    while inicio <= fim {
        let meio = inicio + (fim - inicio)/2;
        match lista[meio].cmp(&alvo) {
            Ordering::Equal => return Some(meio),
            Ordering::Less => inicio = meio + 1,
            Ordering::Greater => {
                if meio == 0 {break;}
                fim = meio - 1
            }
        }
    }    
    None
}

fn main() {
    let lista = [0,14,2,5,77,8];
    let alvo = 77;

    match busca_sequencial(&lista, alvo) {
        Some(num) => println!("Valor encontrado na posição {}", num),
        None => println!("Valor não encontrado")
    };

    let numeros = vec![1,7,200,3,5,9,102,888,1000,600,9,50,120];
    let target = 123;
    match busca_binaria(numeros, target) {
        Some(posicao) => println!("Valor encontrado na posição {}", posicao),
        None => println!("Valor não encontrado")
    };
}