
fn divide(numerador: f64, denominador:f64) -> Result<f64, MathError>{
    if denominador == 0.0{
        return Err(MathError::DivisionByZero);
    }
    if numerador < 0.0 || denominador < 0.0{
        return Err(MathError::NegativeNumber);
    }
    Ok(numerador/denominador)
}

#[derive(Debug, thiserror::Error)]
enum MathError {
    #[error("Erro: Divisão por zero!")]
    DivisionByZero,
    #[error("Erro: Numero negativo!")]
    NegativeNumber
}


fn main(){
    match divide(5.0, 0.0) {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(e) => println!("{}", e),
    }
}