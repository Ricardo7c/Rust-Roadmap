fn main(){
    let lista = vec![1, 2, 3, 4, 5, 6];
    let mut pares = vec![];
    for num in lista{
        if num%2 == 0{
            pares.push(num);
        }
    }
    println!("Números pares: {:?}", pares);
}