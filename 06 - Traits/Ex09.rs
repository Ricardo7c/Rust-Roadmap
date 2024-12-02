trait Ordenavel {
    fn ordenar(&self) -> Self;
}

impl Ordenavel for Vec<i32> {
    fn ordenar(&self) -> Self {
        let mut x = self.clone();
        x.sort();
        x
    }
}

fn ordenar_elementos<T: Ordenavel>(vetor:T) -> T{
    vetor.ordenar()
}

fn main(){
    let lista = vec![5,6,9,3,1,8,0];
    let ordanado = ordenar_elementos(lista);
    println!("{:?}", ordanado);
}
