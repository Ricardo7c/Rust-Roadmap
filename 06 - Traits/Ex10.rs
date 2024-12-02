trait Convertivel {
    fn converter(&self) -> f64;
}

struct Celsius{
    valor: f64
}
struct Fahrenheit{
    valor: f64
}

impl Convertivel for Celsius {
    fn converter(&self) -> f64 {
        self.valor * (9.0/5.0) + 32.0
    }
}

impl Convertivel for Fahrenheit {
    fn converter(&self) -> f64 {
        (self.valor - 32.0) * (5.0/9.0)
    }
}

fn converter_temperatura<T: Convertivel>(temp: T) -> f64{
    temp.converter()
}

fn main(){
    let celcio = Celsius{
        valor: 30.0
    };
    let farhe = Fahrenheit{
        valor: 86.0
    };
    println!("{}° Celcios equivalem a {}° Fahrenheit", celcio.valor.clone(), converter_temperatura(celcio));
    println!("{}° Fahrenheit equivalem a {}° Celcios",farhe.valor.clone(), converter_temperatura(farhe));
}