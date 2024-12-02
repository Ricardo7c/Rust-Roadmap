struct Carro {
    modelo: String,
    ano: u32,
    preco: f64
}

impl Carro {
    fn altera_preco(&mut self, preco:f64){
        self.preco = preco;
    }

    fn info(&self){
        println!("Carro: {} - Ano:{} - R${:.2}", self.modelo, self.ano, self.preco);
    }
}

fn main(){
    let mut fiat = Carro {
        modelo: String::from("Fastback"),
        ano: 2022,
        preco: 70_000.00
    };
    fiat.altera_preco(50_000.00);
    fiat.info();
}