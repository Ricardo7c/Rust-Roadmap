trait Descritivo{
    fn descricao(&self);
}

struct Carro {
    marca: String,
    modelo: String
}

struct Mesa{
    comprimento: f64,
    altura: f64
}

impl Descritivo for Carro {
    fn descricao(&self) {
        println!("Informações do Carro: ");
        println!("Marca: {}",self.marca);
        println!("Modelo: {}", self.modelo);
    }
}

impl Descritivo for Mesa {
    fn descricao(&self) {
        println!("Informações da mesa: ");
        println!("Comprimento: {}", self.comprimento);
        println!("Altura: {}", self.altura);
    }
}

fn main(){
    let car1 = Carro{
        marca: "Honda".to_string(),
        modelo: "Civic".to_string()
    };

    let car2 = Carro{
        marca: "Citroen".to_string(),
        modelo: "Xsara Picasso".to_string()
    };

    let mesa = Mesa{
        comprimento: 210.0,
        altura: 70.0
    };

    car1.descricao();
    car2.descricao();
    mesa.descricao();
}