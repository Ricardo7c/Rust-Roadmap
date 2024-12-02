fn div_segura(num1:f32, num2:f32) -> Result<f32, String>{
    if num2 != 0.0{
        Ok(num1/num2)
    }else{
        Err("Divisão por 0 é impossivel".to_string())
    }
}

fn main(){
    let num1 = 5.0;
    let num2 = 2.0;
    match div_segura(num1, num2) {
        Ok(valor) => println!("{}", valor),
        Err(valor) => println!("{}", valor)
    }
}