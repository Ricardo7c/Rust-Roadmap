enum Mensagem {
    Texto(String),
    Imagem(String, u32, u32),
    Video(String, u32)
}

fn info(msg:Mensagem){
    match msg {
        Mensagem::Texto(conteudo) => println!("{}", conteudo),
        Mensagem::Imagem(titulo, largura , altura ) => println!("{} - {}x{}", titulo, largura, altura),
        Mensagem::Video(titulo,duracao ) => println!("{} - Duração: {}min", titulo, duracao),
    }
}


fn main(){
    let texto = Mensagem::Texto(String::from("Olá, mundo!"));
    let imagem = Mensagem::Imagem(String::from("Praia"), 1280, 720);
    let video = Mensagem::Video(String::from("Cursos"), 25);
    
    info(texto);
    info(imagem);
    info(video);
}