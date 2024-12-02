enum EstadoLampada{
    Ligada,
    Desligada
}

fn lampada(lampada: EstadoLampada){
    match lampada {
        EstadoLampada::Ligada => println!("Lampada ligada!"),
        EstadoLampada::Desligada => println!("Lampada desligada!")
    }
}

fn main(){
    let abajur = EstadoLampada::Ligada;
    let luminaria = EstadoLampada::Desligada;
    lampada(abajur);
    lampada(luminaria);
}