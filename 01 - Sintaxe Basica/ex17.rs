fn main(){
    for n1 in 1..=5{
        for n2 in 1..=5{
            print!("{:3}", n1*n2); //Use print! para não ter quebra de linha dentro do loop interno
        }
        println!(""); //Use um println! vazio para ter uma quebra de linha em cada loop externo
    }
}