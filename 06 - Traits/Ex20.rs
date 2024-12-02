trait Notificacao{
    fn enviar(&self);
}
struct Email{}
struct Sms{}

impl  Notificacao for Email{
    fn enviar(&self) {
        println!("Enviando e-mail de notificação.");
    }
}
impl Notificacao for Sms {
    fn enviar(&self) {
        println!("Enviando SMS de notificação.")
    }
}

fn enviar_notificacoes(notificacoes: Vec<Box<dyn Notificacao>>){
    for notificacao in notificacoes{
        notificacao.enviar();
    }
}


fn main(){
    let email = Box::new(Email{});
    let sms = Box::new(Sms{});

    let notificacoes: Vec<Box<dyn Notificacao>> = vec![email, sms];

    enviar_notificacoes(notificacoes);
}