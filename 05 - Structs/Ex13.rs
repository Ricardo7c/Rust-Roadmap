enum Sinal{
    Red,
    Yellow,
    Green
}

fn next_light(x: Sinal) -> Sinal{
    match x {
        Sinal::Red => Sinal::Green,
        Sinal::Green => Sinal::Yellow,
        Sinal::Yellow => Sinal::Red
    }
}

fn main(){
    let luz_atual = Sinal::Green;

    let proxima = next_light(luz_atual);
    match proxima {
        Sinal::Red => println!("Sinal vermelho: PARE!"),
        Sinal::Yellow => println!("Sinal amarelo: Atenção"),
        Sinal::Green => println!("Sinal verde: Pode seguir!"),
    }
}