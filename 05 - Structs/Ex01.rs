struct Livro{
    titulo: String,
    autor: String,
    paginas: i32
}

impl Livro{
    fn new(titulo: String, autor: String, paginas: i32) -> Livro {
        Livro {
            titulo: titulo,
            autor: autor,
            paginas: paginas
        }
    }
}

fn main(){
    let harry_potter = Livro::new(String::from("Harry potter"), String::from("J.K. Roling"), 450);
    println!("Titulo: {}", harry_potter.titulo);
    println!("Autor: {}", harry_potter.autor);
    println!("Numero de paginas: {}", harry_potter.paginas);
}