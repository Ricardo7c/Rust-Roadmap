trait Descrevivel {
    fn descricao(&self) -> String;
}
trait Identificavel: Descrevivel {
    fn identificar(&self) -> String {
    format!("Identificação: {}", self.descricao())
    }
}


struct Produto{
    nome: String,
    preco: f64
}

impl Descrevivel for Produto {
    fn descricao(&self) -> String {
        format!("Produto: {} - Preço: R${}", self.nome, self.preco)
    }
}

impl Identificavel for Produto {}

fn main(){
    let notebook = Produto{
        nome: "Acer".to_owned(),
        preco: 5000.00
    };

    println!("{}", notebook.descricao());
    println!("{}", notebook.identificar());
}