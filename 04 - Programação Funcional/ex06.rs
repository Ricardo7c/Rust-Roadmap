fn convert(num: String) -> Result<i32, String>{
    match num.trim().parse::<i32>() {
        Ok(valor) => Ok(valor),
        Err(_) => Err("Erro, falha ao converter".to_owned())
    }
}

fn main(){
    let num = String::from("35");
    match convert(num) {
        Ok(valor) => println!("{}", valor+6),
        Err(erro) => println!("{}", erro)
    }

    let num2 = String::from("Oi");
    match convert(num2) {
        Ok(valor) => println!("{}", valor+6),
        Err(erro) => println!("{}", erro)
    }
}