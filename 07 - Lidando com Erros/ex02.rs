use std::num::ParseIntError;

fn parse_number(entrada: String) -> Result<i32, ParseIntError> {
    entrada.trim().parse::<i32>()
}


fn calcular_square(entrada: String) -> Result<i32, ParseIntError> {
    let num = parse_number(entrada)?; // Propaga erro se `parse_number` falhar
    Ok(num*num) // Retorna o valor se nenhum erro for encontrado
}

fn main(){
    let entrada = "aaa".to_string();

    match calcular_square(entrada) {
        Ok(valor) => println!("O quadado é: {}", valor),
        Err(e) => println!("Erro: {}", e)
    }
}