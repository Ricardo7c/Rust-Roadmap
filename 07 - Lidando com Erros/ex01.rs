use std::fs::File;
use std::io::Read;

fn read_file(arquivo: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(arquivo)?;
    let mut conteudo = String::new();
    file.read_to_string(&mut conteudo)?;
    Ok(conteudo)
}

fn main() {
    let arquivo = "Arquivo.txt";
    match read_file(arquivo) {
        Ok(conteudo) => println!("{}", conteudo),
        Err(x) => println!("{:?}", x)
    }
}
