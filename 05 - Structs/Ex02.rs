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

    fn resumo(&self) -> String{
        format!("Livro: {} by {}", self.titulo, self.autor)
    }
}

fn main(){
    let harry_potter = Livro::new(
        String::from("Harry potter"), 
        String::from("J.K. Roling"), 
        450);
    
    let resumo = harry_potter.resumo();
    println!("{}", resumo);
    println!("Numero de paginas: {}", harry_potter.paginas);
}