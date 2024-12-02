const PI :f64 = 3.14;
trait Area {
    fn calcular_area(&self) -> f64;
}
struct Retangulo{
    largura: f64,
    altura: f64
}
struct Circulo{
    raio: f64,
}
impl Area for Retangulo {
    fn calcular_area(&self) -> f64 {
        self.altura*self.largura
    }
}
impl Area for Circulo {
    fn calcular_area(&self) -> f64 {
        self.raio*self.raio*PI
    }
}

fn mostrar_area<T: Area>(objeto:&T){
    println!("A Area é {}", objeto.calcular_area());
}

fn main(){
    let retangulo = Retangulo {
        largura: 10.0,
        altura: 20.0
    };
    let circulo = Circulo{
        raio: 15.0
    };

    mostrar_area(&retangulo);
    mostrar_area(&circulo);
}