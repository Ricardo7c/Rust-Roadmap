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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisao_valida() {
        //Testa com valores corretos
        let resultado = divide(10.0, 2.0);
         // Se o resultado for OK
        assert!(resultado.is_ok());
        // Verifica se o resultado é o esperado         
        assert_eq!(resultado.unwrap(), 5.0); 
    }

    #[test]
    fn test_divisao_por_zero() {
        // Testa com denominador 0
        let resultado = divide(10.0, 0.0);
        // Se o resultado for um erro
        assert!(resultado.is_err());
        // Verifica se a messagem de erro foi a esperada
        if let Err(err) = resultado {
            assert_eq!(format!("{}", err), "Erro: Divisão por zero!"); // Confirma a mensagem do erro
        }
    }

    #[test]
    fn test_divisao_com_numero_negativo() {
        // Testa com numero negativo
        let resultado = divide(-10.0, 2.0);
        // Se o resultado for um erro
        assert!(resultado.is_err());
        // Verifica se a msg de erro é a esperada
        if let Err(err) = resultado {
            assert_eq!(format!("{}", err), "Erro: Numero negativo!"); // Confirma a mensagem do erro
        }
}
}