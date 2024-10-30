fn maior_numero(lista: &[i32; 8]) -> &i32{
    let maior = lista.iter().max().unwrap();
    maior
}

fn main(){
    let array = [2, 3, 5, 11, 6, 7, 2, 8];
    let maior = maior_numero(&array);
    println!("{}", maior)
}