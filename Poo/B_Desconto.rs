trait Desconto {
    fn calcular_desconto(&self, desconto: f64) -> f64;
}

struct Produto{
    nome: String,
    preco: f64
}

impl Desconto for Produto {
    fn calcular_desconto(&self, desconto: f64) -> f64 {
        self.preco - (self.preco*(desconto/100.0))
    }
}

impl Produto {
    fn mostrar_produto(&self){
        println!("{} - R${:.2}", self.nome, self.preco);
    }
}

fn main(){
    let leite = Produto{
        nome: "Leite".to_string(),
        preco: 10.0
    };

    leite.mostrar_produto();
    println!("Valor com desconto de 10% : R${:.2}",leite.calcular_desconto(10.0));
}