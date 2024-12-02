enum Notification{
    Email(String, String),
    SMS(String, String),
    PushNotification(String, String)
}


fn send_notification(notificacao:Notification){
    match notificacao {
        Notification::Email(email,assunto ) => {
            println!("Email De: {}", email);
            println!("Assunto: {}", assunto);
        },
        Notification::SMS(numero,msg ) => {
            println!("SMS do numero: {}", numero);
            println!("Mensagem: {}", msg);
        },
        Notification::PushNotification(app,notification ) => {
            println!("Notificação do App: {}", app);
            println!("Conteudo: {}", notification);
        }
    }
}


fn main(){
    let email = Notification::Email("email@rust.com".to_owned(), "E-mail do curso de Rust".to_owned());
    let sms = Notification::SMS("555-6985".to_owned(), "SMS do curso de Rust".to_owned());
    let push = Notification::PushNotification("Rust App".to_owned(), "Notificação do curso de Rust".to_owned());

    
    send_notification(email);
    send_notification(sms);
    send_notification(push);
}