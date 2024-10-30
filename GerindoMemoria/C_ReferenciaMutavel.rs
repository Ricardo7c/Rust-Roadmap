fn completar_saudacao(texto: &mut String){
    texto.push('!');
}

fn main(){
    // Se a variavel não for mutavel, o codigo não vai compilar.
    let mut texto = "Bem vindo".to_string();
    // A referencia tambem precisa ser mutavel
    completar_saudacao(&mut texto);
    println!("{texto}");
}