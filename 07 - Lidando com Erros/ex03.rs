fn divide(numerador: f64, denominador:f64) -> Result<f64, MathError>{
    if denominador == 0.0{
        return Err(MathError::DivisionByZero);
    }
    if numerador < 0.0 || denominador < 0.0{
        return Err(MathError::NegativeNumber);
    }
    Ok(numerador/denominador)
}

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeNumber
}

fn main(){
    match divide(5.0, 0.0) {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(MathError::DivisionByZero) => println!("Erro: Divisão por zero não é permitida!"),
        Err(MathError::NegativeNumber) => println!("Erro: Números negativos não são permitidos!"),
    }
}