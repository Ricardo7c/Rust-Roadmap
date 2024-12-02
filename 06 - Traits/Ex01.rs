trait Descrevivel {
    fn descricao(&self) -> String{
        "Emitir som".to_string()
    }
}

struct Carro{
    marca: String,
    modelo: String
}

impl Descrevivel for Carro {
    fn descricao(&self) -> String {
        format!("Marca:{} -  Modelo:{}", self.marca, self.modelo)
    }
}

fn main(){
    let car = Carro {
        marca: "Honda".to_string(),
        modelo: "Civic".to_string()
    };
    println!("{}",car.descricao());
}