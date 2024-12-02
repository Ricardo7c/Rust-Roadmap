fn maior_numero(lista: Vec<i32>) -> Option<i32>{
    lista.iter().max().copied()
}

fn main(){
    let lista = vec![2,6,8,9,4,1,7];
    match maior_numero(lista) {
        Some(valor) => println!("O maior numero é: {}", valor),
        None => println!("A lista está vazia")
    }
}
