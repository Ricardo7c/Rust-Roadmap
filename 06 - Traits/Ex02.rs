trait Som {
    fn emitir_som(&self){
        println!("Som genérico");
    }
}

struct Cachorro{}

impl Som for Cachorro {
    fn emitir_som(&self) {
        println!("Au Au");
    }
}

fn main(){
    let cachorro = Cachorro{};

    cachorro.emitir_som();
}