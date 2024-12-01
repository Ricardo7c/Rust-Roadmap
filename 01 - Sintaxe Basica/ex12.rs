fn main(){
    let mut cont = 3;
    loop {
        if cont == 0{
            println!("Saiu do loop");
            break;
        }
        println!("Olá, loop");
        cont -= 1;
    }
}