fn main(){
    let mut texto = "Rust".to_string();
    let referencia = &mut texto;
    referencia.push_str(" é incrivel");

    println!("{}",texto);
}
