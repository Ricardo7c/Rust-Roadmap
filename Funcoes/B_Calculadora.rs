fn main(){
    let soma = |a, b| a + b;
    let sub = |a, b| a - b;
    let multi = |a, b| a * b;
    let div = |a, b| {
        if b != 0 {
            Some(a / b)
        } else {
            None
        }
    };


    println!("{}", soma(5,2));
    println!("{}", sub(5,2));
    println!("{}", multi(5,2));
    match div(5,0) {
        Some(resultado) => println!("{}", resultado),
        None => println!("Erro, divisão por zero não é permitida.")
    }
}