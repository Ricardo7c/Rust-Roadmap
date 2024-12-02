use text_input::text;

fn main(){
    let nome = text("Qual seu nome? ");
    println!("Bem vindo(a) {}", nome);

}