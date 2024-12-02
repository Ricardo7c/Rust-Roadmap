fn primeiro_valor<T: Copy>(lista: Vec<T>) -> Option<T>{
    if lista.len() > 0{
        return Some(lista[0])
    }
    None
    
}


fn main(){
    let lista1 = vec![2, 3, 5, 6, 6];
    let lista2 = vec!["oi", "novo", "texto", "teste"];
    let lista3: Vec<i32> = vec![];

    match primeiro_valor(lista1) {
        Some(valor) => println!("O primeiro valor é: {valor}"),
        None => println!("A lista está vazia!")
    }

    match primeiro_valor(lista2) {
        Some(valor) => println!("O primeiro valor é: {valor}"),
        None => println!("A lista está vazia!")
    }

    
    match primeiro_valor(lista3) {
        Some(valor) => println!("O primeiro valor é: {valor}"),
        None => println!("A lista está vazia!")
    }
}