trait Animal{
    fn nome(&self) -> String;
    fn emitir_som(&self) -> String;
}
struct Leao{
    nome: &'static str
}
struct Macaco{
    nome: &'static str
}

impl Animal for Leao {
    fn nome(&self) -> String {
        self.nome.to_string()
    }
    fn emitir_som(&self) -> String {
        "Rugido".to_string()
    }
}

impl Animal for Macaco {
    fn nome(&self) -> String {
        self.nome.to_string()
    }
    fn emitir_som(&self) -> String {
        "Grito".to_string()
    }
}

fn mostrar_animais(animais: Vec<Box<dyn Animal>>){
    for animal in animais{
        println!("Nome: {} - Som: {}", animal.nome(), animal.emitir_som())
    }
}

fn main(){
    let a1 = Box::new(Macaco{
        nome: "Cesar"
    });
    let a2 = Box::new(Leao{
        nome: "Simba"
    });

    let animais:Vec<Box<dyn Animal>> = vec![a1, a2];

    mostrar_animais(animais);
}