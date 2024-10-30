fn adicionar(pilha: &mut Vec<i32>, num: i32){
    pilha.push(num);
}

fn remover(pilha: &mut Vec<i32>) -> Option<i32>{
    pilha.pop()
}

fn mostrar_pilha(pilha: &Vec<i32>){
    for cada in pilha{
        print!("{} ", cada)
    }
}

fn main(){
    let mut pilha: Vec<i32> = vec![];
    adicionar(&mut pilha, 1);
    adicionar(&mut pilha, 4);
    adicionar(&mut pilha, 3);
    adicionar(&mut pilha, 2);
    if let Some(valor) = remover(&mut pilha){
        println!("Valor removido: {}", valor);
    }else{
        println!("lista esta vazia!")
    }
    mostrar_pilha(&pilha);
}