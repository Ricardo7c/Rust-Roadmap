
trait Positivo {
    fn eh_positivo(&self) -> bool;
}

impl Positivo for i32 {
    fn eh_positivo(&self) -> bool {
        *self > 0
    }
}
// O uso do Trait Bound força a função a aceitar apenas iteraveis de i32
fn filtrar_positivos<T: IntoIterator<Item = i32>> (numeros: T) -> Vec<i32> {
    let mut x = vec![];
    for num in numeros{
        if num.eh_positivo(){
            x.push(num);
        }
    }
    x
}

fn main(){
    let numeros = [5,-2,-6,3,9];
    let positivos = filtrar_positivos(numeros);
    println!("{:?}", positivos);

}