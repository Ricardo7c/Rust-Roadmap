trait Exibivel{
    fn exibir(&self);
}

trait Comparavel: Exibivel {
    fn comparar(&self, outro: &Self) -> bool;
}

struct Carro {
    marca: String,
    velocidade: u32
}

impl Comparavel for Carro {
    fn comparar(&self, outro: &Self) -> bool {
        self.velocidade == outro.velocidade
    }
}


impl Exibivel for Carro {
    fn exibir(&self){
        println!("Carro: {} com velocidade de {} km/h", self.marca, self.velocidade)
    }
}

fn comparar_carros<T: Comparavel>(car1: &T, car2: &T){
    car1.exibir();
    car2.exibir();
    if car1.comparar(&car2){
        println!("Os carros estão na mesma velocidade");
    }else{
        println!("Os carros estão em velocidades diferentes");
    }
}


fn main(){
    let car1 = Carro{
        marca: "Honda".to_string(),
        velocidade: 150
    };
    let car2 = Carro{
        marca: "Celta".to_string(),
        velocidade: 150
    };

    comparar_carros(&car1, &car2);

}