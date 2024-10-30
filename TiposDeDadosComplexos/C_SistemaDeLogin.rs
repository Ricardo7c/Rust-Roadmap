use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{} ",texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x.trim().to_string().to_uppercase()
}
enum Logged {
    Ativo,
    Desativado,
    Bloqueado,
}
trait Login{
    fn fazer_login(&self, login: String, senha: String) -> Logged;
}
struct Pessoa{   
    login: String,
    senha: String
}
impl Login for Pessoa {
    fn fazer_login(&self, login: String, senha: String) -> Logged{
        if self.login == login && self.senha == senha{
            return Logged::Ativo
        }else{
            return Logged::Desativado
        }
    }
}

fn main(){
    let ricardo = Pessoa{
        login: "Ricardo".to_string().to_uppercase(),
        senha: "pass".to_string().to_uppercase()
    };

    let mut tentativas = 3;
    loop{
        let logado: Logged;
        if tentativas >= 1{
            println!("--- Sistema de login ---");
            let login = input("Digite o nome de usuario: ");
            let senha = input("Digite a senha: ");
            logado = ricardo.fazer_login(login, senha);
        }else{
            logado = Logged::Bloqueado;
        }

        tentativas -= 1;
        
        match logado {
            Logged::Ativo => {
                println!("Login efetuado com sucesso!");
                break;
            },
            Logged::Desativado => {
                println!("Usuario ou senha invalidos, tente novamente!");
                println!("Numero de tentativas: {} ", tentativas);
                continue;
            },
            Logged::Bloqueado => {
                println!("Ultrapassou o numero de tentativas o login foi bloqueado!");
                break;
            }
        }
    }

}