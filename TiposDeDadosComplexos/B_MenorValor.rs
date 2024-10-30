fn menor_numero(lista: &[i32]) -> Option<&i32>{
    lista.iter().min()
}

fn main(){
    let array = [2, 3, 5, 11, 6, 7, 2, 8, 58];
    if let Some(valor) = menor_numero(&array){
        println!("O menor valor da lista é: {}", valor);
    }else{
        println!("A lista está vazia!");
    }
}