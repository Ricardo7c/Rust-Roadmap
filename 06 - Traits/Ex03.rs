trait Movimento {
    fn mover(&self);
}

struct Carro {}
struct Bicicleta{}

impl Movimento for Carro {
    fn mover(&self) {
        println!("O Carro está se movendo!");
    }
}

impl Movimento for Bicicleta {
    fn mover(&self) {
        println!("A Bicicleta está se movendo!");
    }
}

fn main(){
    let carro = Carro{};
    let bicicleta = Bicicleta{};
    
    carro.mover();
    bicicleta.mover();
}