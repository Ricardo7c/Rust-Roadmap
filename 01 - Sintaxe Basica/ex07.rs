const PI :f32 = 3.14;
fn main(){
    let raio:i32 = 5;
    let area:f32 = PI*(raio.pow(2)) as f32;
    println!("A area de um circulo com raio {} é: {}", raio, area);
}
