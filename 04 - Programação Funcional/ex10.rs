fn validacao(idade: i32) -> Result<String, String>{
    if idade >= 18{
        Ok("Maior de idade!".to_string())
    }else{
        Err("Menor de idade!".to_string())
    }
}

fn main(){
    let maioridade = validacao(21);
    match maioridade {
        Ok(valor) => println!("{valor}"),
        Err(valor) => println!("{valor}")
    }
}