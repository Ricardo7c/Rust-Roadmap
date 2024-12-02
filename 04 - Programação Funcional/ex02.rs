fn menor_palavra<'a>(texto: &'a str) -> &'a str{
    texto.split_whitespace().min_by_key(|palavra| palavra.len()).unwrap()
}

fn main(){
    let texto = "Programando em Rust";
    println!("A menor palavra => {}",menor_palavra(texto));
}