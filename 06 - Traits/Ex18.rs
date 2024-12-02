trait Sensor{
    fn ler_dado(&self) -> String;
}

struct SensorTemperatura{
    temperatura: f64
}
struct SensorUmidade{
    umidade: &'static str
}

impl Sensor for SensorTemperatura {
    fn ler_dado(&self) -> String {
        format!("Temperatura: {}", self.temperatura)
    }
}

impl Sensor for SensorUmidade {
    fn ler_dado(&self) -> String {
        format!("Umidade: {}", self.umidade)
    }
}

fn coletar_dados(sensores: Vec<Box<dyn Sensor>>){
    for sensor in sensores{
        println!("{}", sensor.ler_dado())
    }
}


fn main(){
    let t1 = Box::new(SensorTemperatura{
        temperatura: 23.6
    });
    let t2 = Box::new(SensorTemperatura{
        temperatura: 32.0
    });
    let u1 = Box::new(SensorUmidade{
        umidade: "66%"
    });
    let u2 = Box::new(SensorUmidade{
        umidade: "70%"
    });

    let sensores: Vec<Box<dyn Sensor>> = vec![t1, u1, t2, u2];

    coletar_dados(sensores);

}