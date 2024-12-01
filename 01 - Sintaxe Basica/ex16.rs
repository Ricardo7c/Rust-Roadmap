fn main(){
    let numero = 5;
    match numero {
        ..=10 => println!("Pequeno"),
        11..=20 => println!("Medio"),
        21.. => println!("Grande")
    }
}

