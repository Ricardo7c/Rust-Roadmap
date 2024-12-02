enum Result {
    Success(String),
    Error(String)
}

fn process_result(x: Result){
    match x {
        Result::Success(conectado) => println!("{conectado}"),
        Result::Error(falha) => println!("{falha}")
    }
}

fn main(){
    let conectado = Result::Success("Conexão com o banco de dados realizada com sucesso!".to_owned());
    let erro = Result::Error("Falha na conexão com o banco de dados".to_owned());

    process_result(conectado);
    process_result(erro);
}