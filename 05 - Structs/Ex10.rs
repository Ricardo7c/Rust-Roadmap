enum EstatusDePagamento{
    Pendente(String),
    Completo(f64),
    Falhou(String)
}

fn print_status(pagamento: EstatusDePagamento){
    match pagamento {
        EstatusDePagamento::Pendente(pag) => println!("{}", pag),
        EstatusDePagamento::Falhou(pag) => println!("{}", pag),
        EstatusDePagamento::Completo(pag) => println!("O valor pago foi de: {:.2}", pag)
    }
}

fn main(){
    let pagamento = EstatusDePagamento::Pendente("Você ainda não fez o pagamento".to_owned());
    let falha = EstatusDePagamento::Falhou("Erro: Pagamento não realizado".to_owned());
    let completo = EstatusDePagamento::Completo(532.0);

    print_status(pagamento);
    print_status(falha);
    print_status(completo);
}