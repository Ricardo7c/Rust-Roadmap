#[derive(Clone, Copy, Debug)]
enum Cor{
    _Vermelho,
    _Verde,
    _Azul
}

fn main(){
    let c1 = Cor::_Vermelho;
    let c2 = c1.clone();

    println!("{:?}", c1);
    println!("{:?}", c2);
}
