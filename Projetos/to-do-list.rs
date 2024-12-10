use std::io::{self, Write};
use std::process::Command;

struct Tarefa{
    nome: String,
    status: bool
}

fn input(texto:&str) -> String{ //Entrada de texto
    print!("{texto}");
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x.trim().to_string()

}

fn exibir_lista(lista: &Vec<Tarefa>){ // Exbie a lista
    if !lista.is_empty(){
        println!("===== Lista de Tarefas =======");
        for (id, task) in lista.iter().enumerate() {
            if task.status == true{
                println!("| {} - {} - (Concluida)", id+1, task.nome);
                continue;
            }
            println!("| {} - {}", id+1, task.nome);
        }
    }else{
        println!("A lista está vazia!");
    }
    println!("==============================")
    
}

fn add(lista: &mut Vec<Tarefa> ){ // Adiciona novas tarefas a lista
    let nome = input("Digite a tarefa: ");
    if !nome.is_empty(){
        lista.push(Tarefa{nome, status: false});
    }
    
}

fn del(lista: &mut Vec<Tarefa>){ // Remove uma tarefa em um indice especifico
    loop{
        let indice = input("Digite o numero da tarefa que deseja remover: ");
        if indice.is_empty(){
            break;
        }
        match indice.parse::<usize>() {
            Ok(indice) => {
                if indice-1 <= lista.len(){
                    lista.remove(indice-1);
                    break;
                }else{
                    println!("Digite um numero valido!");
                }
            },
            Err(_) => println!("Valor invalido!")
        }
    }
}

fn limpar_tela() {  // Limpa a tela anterior no inicio de cada loop
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn concluir(lista: &mut Vec<Tarefa>){
    loop{
    let tarefa = input("Digite o numero da tarefa concluida: ");
    if tarefa.is_empty(){
        break;
    }
    match tarefa.parse::<usize>() {
        Ok(indice) => {
            if indice-1 <= lista.len(){
                lista[indice-1].status = true;
                break;
            }else{
                println!("Digite um numero valido!");
            }
        },
        Err(_) => println!("Valor invalido!")
    }
    }
}

fn main(){
    let mut lista :Vec<Tarefa> = vec![];
    loop{
        limpar_tela();
        exibir_lista(&lista);
        println!();
        println!("A) Adicionar");
        println!("B) Concluir");
        println!("C) Remover");
        let opcao = input("Digite a tarefa que deseja adicionar<ENTER> para remover: ");
        match opcao.as_str() {
            "A" => add(&mut lista),
            "B" => concluir(&mut lista),
            "C" => del(&mut lista),
            _ => println!("Opção invalida, tente novamente!")
            }
        }
    }
    