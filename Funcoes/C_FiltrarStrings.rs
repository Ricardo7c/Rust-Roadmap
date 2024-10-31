fn filtrar_string(lista: Vec<String>) -> Vec<String>{
    let x = lista.into_iter().filter(|fruta| fruta.starts_with('M')).collect();
    x
}

fn main(){
    let frutas = vec!["Maçã".to_string(), "Banana".to_string(), "Laranja".to_string(), 
    "Manga".to_string(), "Abacaxi".to_string(), "Melancia".to_string(), "Morango".to_string()];
    for fruta in filtrar_string(frutas){
        println!("{}", fruta);
    }
}