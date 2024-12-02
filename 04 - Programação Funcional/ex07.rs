fn buscar_nome(lista: Vec<String>, nome: &String) -> Option<usize>{
    for (a, b) in lista.iter().enumerate(){
        if b == nome{
            return Some(a);
        }
    }
    None
}
fn main(){
    let lista = vec!["Ricarodo".to_string(), "Pedro".to_string(), "Laura".to_string()];
    let nome = "João".to_string();
    let busca = buscar_nome(lista, &nome);
    match busca {
        Some(valor) => println!("{}", valor),
        None => println!("Nome não encontrado!")
    }
}