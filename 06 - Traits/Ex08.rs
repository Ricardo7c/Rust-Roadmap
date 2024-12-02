trait Comparavel {
    fn maior_que(self, outro: Self) -> bool;
}

struct Pessoa{
    idade: u32
}

impl Comparavel for Pessoa {
    fn maior_que(self, outro: Self) -> bool{
        self.idade > outro.idade
    }
}

fn comparar_pessoas<T: Comparavel>(p1: T, p2: T){
    if p1.maior_que(p2){
        println!("A primeira pessoa é maior");
    }else{
        println!("A segunda pessoa é maior");
    }
}

fn main(){
    let p1 = Pessoa{
        idade: 23
    };
    let p2 = Pessoa{
        idade: 12
    };

    comparar_pessoas(p1, p2);
}