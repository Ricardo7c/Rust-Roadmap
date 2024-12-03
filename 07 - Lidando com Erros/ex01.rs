use std::fs;

fn read_file(arquivo: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(arquivo)
}

fn main() {
    let arquivo = "Arquivo.txt";
    match read_file(arquivo) {
        Ok(conteudo) => println!("{}", conteudo),
        Err(err) => eprintln!("Erro: {}", err),
    }
}