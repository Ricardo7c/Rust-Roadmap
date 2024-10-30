fn filtrar(lista: Vec<i32>) -> Vec<i32>{
    lista.into_iter().filter(|x| x % 2 == 0).collect()
}

fn main(){
    let lista = vec![2, 3, 6, 9, 11, 5];
    let filtrados = filtrar(lista);
    println!("{:?}",filtrados);
}