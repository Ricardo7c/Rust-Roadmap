struct Pessoa{
    nome: String,
    endereco: Endereco,
}

struct Endereco{
    rua: String,
    numero: u32,
    cidade: String,
}

impl Pessoa {
    fn exibir_info(&self){
        println!("Nome: {}", self.nome);
        println!("Endereço: {} - {} - {}", self.endereco.rua, self.endereco.numero, self.endereco.cidade);
    }
}

fn main(){
    let endereco = Endereco{
        rua: String::from("C. H. Hutmacher"),
        numero: 1048,
        cidade: String::from("Miguel pereira")
    };

    let ricardo = Pessoa{
        nome: String::from("Ricardo"),
        endereco,
    };

    ricardo.exibir_info();

}