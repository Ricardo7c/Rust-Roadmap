#[derive(PartialEq)]
struct Pessoa{
    _nome: String,
    _idade: u8
}

fn main(){
    let p1 = Pessoa{
        _nome: "Ricardo".to_string(),
        _idade: 34
    };

    let p2 = Pessoa{
        _nome: "Claudia".to_string(),
        _idade: 50
    };

    if p1 == p2{
        println!("As duas pessoas são iguais")
    }else{
        println!("As duas pessoas são diferentes")
    }
}