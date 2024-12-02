trait Animal{
    fn comer(&self);
}
trait Mamifero: Animal{
    fn andar(&self);
}
struct Humano{}

impl Animal for Humano {
    fn comer(&self) {
        println!("O humano está comendo!");
    }
}

impl Mamifero for Humano {
    fn andar(&self) {
        println!("O humano está andando!");
    }
}

fn main(){
    let pessoa = Humano{};

    pessoa.comer();
    pessoa.andar();
}