trait Nomeavel{
    fn nome(&self) -> String;
}

trait ComparavelPorNome: Nomeavel {
    fn comparar_nomes(&self, outro: &Self);
}

impl Nomeavel for Pessoa {
    fn nome(&self) -> String {
        format!("{}", self.nome)
    }
}

impl ComparavelPorNome for Pessoa {
    fn comparar_nomes(&self, outro: &Self) {
        if self.nome() == outro.nome(){
            println!("Os nomes são iguais!");
        }else {
            println!("Os nomes são diferentes!");
        }
    }
}

struct Pessoa{
    nome: &'static str
}

fn comparar_pessoas<T: ComparavelPorNome>(pessoa1: &T, pessoa2: &T ){
    pessoa1.comparar_nomes(pessoa2);
}

fn main(){
    let p1 = Pessoa{
        nome: "Ricardo"
    };

    let p2 = Pessoa{
        nome: "Ricardo"
    };

    let p3 = Pessoa{
        nome: "ChatGPT"
    };

    comparar_pessoas(&p1, &p2);
    println!("==================");
    comparar_pessoas(&p1, &p3);
}