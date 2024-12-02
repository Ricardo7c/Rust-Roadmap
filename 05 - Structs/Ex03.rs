struct Coordenada (f64, f64);

impl Coordenada {
    fn mostrar_local(&self){
        println!("Latitude: {}", self.0);
        println!("Longitude: {}", self.1)
    }
}

fn main(){
    let local = Coordenada(-28.945366460321747, -14.833819494180254);
    local.mostrar_local();
}