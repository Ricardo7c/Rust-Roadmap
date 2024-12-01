fn main(){
    let lista = [3, 1, 4, 1, 5, 9];
    let mut ordenado = lista.to_vec();
    ordenado.sort();

    println!("Array ordenado: {:?}", ordenado);
}