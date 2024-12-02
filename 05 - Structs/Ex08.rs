enum Veiculo{
    Carro(DetalhesCarro),
    Moto(DetalhesMoto),
}

struct DetalhesCarro {
    cor: String,
    vidros: String,
}

struct DetalhesMoto {
    cor: String,
    adesivo: String,
}

fn info(veiculo: Veiculo){
    match veiculo {
        Veiculo::Carro(carro) => println!("Cor: {} - Vidros: {}", carro.cor, carro.vidros),
        Veiculo::Moto(moto) => println!("Cor: {} - Adesivos: {}", moto.cor, moto.adesivo)
    }
}


fn main(){
    let carro  = Veiculo::Carro(DetalhesCarro{
        cor: "Azul".to_string(),
        vidros: "Blindados".to_string()
    });

    let moto = Veiculo::Moto(DetalhesMoto{
        cor: "Preta".to_string(),
        adesivo:"Fogo".to_string()
    });

    info(carro);
    info(moto);
}