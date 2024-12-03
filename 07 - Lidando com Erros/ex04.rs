use std::fmt;

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

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Erro: Divisão por zero!"),
            MathError::NegativeNumber => write!(f, "Erro: Numero negativo!")
        }
    }
}


fn main(){
    match divide(5.0, 0.0) {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(e) => println!("{}", e),
    }
}