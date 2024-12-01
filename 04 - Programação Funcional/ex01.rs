fn maior(n1:i32, n2:i32) -> i32{
    if n1 > n2{
        return n1;
    }
    n2
}

fn main(){
    println!("O maior é: {}", maior(2, 4));
    println!("O maior é: {}", maior(8, 1));
    println!("O maior é: {}", maior(19, 45))
}