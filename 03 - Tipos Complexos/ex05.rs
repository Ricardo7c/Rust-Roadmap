fn main(){
    let lista = vec![("Maçã", 2.5), ("Banana", 1.5), ("Pêra", 3.0)];
    let mut soma = 0.0;
    for cada in lista{
        soma += cada.1
    }
    
    println!("Preço total: {}", soma);

}
