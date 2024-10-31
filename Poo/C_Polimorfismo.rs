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
    let mut objetos: Vec<Box<dyn Descritivo>> = Vec::new();

    let car1 = Box::new(Carro{
        marca: "Honda".to_string(),
        modelo: "Civic".to_string()
    });

    let car2 = Box::new(Carro{
        marca: "Citroen".to_string(),
        modelo: "Xsara Picasso".to_string()
    });

    let mesa = Box::new(Mesa{
        comprimento: 210.0,
        altura: 70.0
    });

    objetos.push(car1);
    objetos.push(car2);
    objetos.push(mesa);

    for objeto in objetos{
        objeto.descricao();
    }
}