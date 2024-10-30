struct Pessoa{
    nome: String,
    idade: u8
}

impl Pessoa{
    fn mudar_nome(&mut self, nome: &str){
        self.nome = nome.to_string();
    }
    fn mudar_idade(&mut self, idade: u8){
        self.idade = idade;
    }
    fn mostrar_info(&self){
        println!("{} - {}", self.nome, self.idade);
    }
}

fn main(){
    let mut pessoa = Pessoa{
        nome: "Ricardo".to_string(),
        idade: 30
    };

    pessoa.mudar_nome("Ricardo Silva");
    pessoa.mudar_idade(34);

    pessoa.mostrar_info();

}
