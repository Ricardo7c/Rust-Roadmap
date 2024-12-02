#[derive(Debug)]
struct Livro {
    _titulo: String,       //Iniciei os atributos com _ só para evitar Warnings, não é nescessario.
    _autor: String,
    _paginas: u32
}

fn main(){
    let livro1 = Livro{
        _titulo: "1984".to_string(),
        _autor: "George Orwell".to_string(),
        _paginas: 328
    };

    println!("{:?}", livro1)
}