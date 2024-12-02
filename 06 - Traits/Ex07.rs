trait Somavel {
    fn somar(&self, num: &Self) -> Self;
}

impl Somavel for i32 {
    fn somar(&self, num: &i32) -> i32 {
        *self+*num
    }
}

impl Somavel for f64 {
    fn somar(&self, num: &f64) -> f64 {
        *self+*num
    }
}

fn somar_valores<T: Somavel>(n1: T, n2: T ) -> T{
    n1.somar(&n2)
}

fn main() {
    let soma = somar_valores(5.5, 2.3);
    println!("A Soma é: {}", soma);
}