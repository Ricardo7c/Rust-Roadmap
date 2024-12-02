trait Movimento{
    fn mover(&self);
}

trait MovimentoDescritivo: Movimento {
    fn mover_com_descricao(&self);
}

struct Carro {
    cor: &'static str,
    velocidade: f64
}
struct Aviao{
    cor: &'static str,
    velocidade: f64
}

impl Movimento for Carro {
    fn mover(&self) {
        println!("O Carro está em movimento")
    }
}

impl Movimento for Aviao {
    fn mover(&self) {
        println!("O avião está voando")
    }
}

impl MovimentoDescritivo for Carro {
    fn mover_com_descricao(&self) {
        self.mover();
        println!("O carro {} está se movendo a {} km/h", self.cor, self.velocidade)
    }
}

impl MovimentoDescritivo for Aviao {
    fn mover_com_descricao(&self) {
        self.mover();
        println!("O avião {} está voando a {} km/h", self.cor, self.velocidade)
    }
}

fn mover_veiculo_com_descricao <T: MovimentoDescritivo>(veiculo: T){
    veiculo.mover_com_descricao();
    
}


fn main(){
    let carro = Carro {
        cor: "Preto",
        velocidade: 150.0
    };
    let aviao = Aviao {
        cor: "Branco",
        velocidade: 350.0
    };

    mover_veiculo_com_descricao(carro);
    println!("===============================");
    mover_veiculo_com_descricao(aviao);
} 