const PI: f64 = 3.14;

trait CalculoArea{
    fn area(&self) -> f64;
}

trait FormaExibivel: CalculoArea {
    fn exibir_area(&self);
}

impl FormaExibivel for Quadrado {
    fn exibir_area(&self) {
        println!("A Area do Quadrado é: {:.2} m²", self.area());
    }
}

impl FormaExibivel for Circulo {
    fn exibir_area(&self) {
        println!("A Area do Ciruculo é: {:.2} m", self.area());
    }
}

impl CalculoArea for Quadrado {
    fn area(&self) -> f64 {
        self.lados*2.0
    }
}

impl CalculoArea for Circulo {
    fn area(&self) -> f64 {
        PI*(self.raio*self.raio)
    }
}

fn calcular_area<T: FormaExibivel>(forma: T){
    forma.exibir_area();
}


struct Quadrado{
    lados: f64
}

struct Circulo{
    raio: f64
}

fn main(){
    let quad = Quadrado{
        lados: 2.0
    };

    let circulo = Circulo{
        raio: 3.25
    };

    calcular_area(quad);
    calcular_area(circulo);
}