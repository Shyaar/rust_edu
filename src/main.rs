
#[derive(Debug)]
#[allow(unused,unused_variables)]

struct Credentials{
    username: String,
    password: String,
}
#[derive(Debug)]

enum Payment {
    CreditCard(String),
    DebitCard(Credentials),
    Wallet(String),
}
fn main(){

    let credents = Credentials{
        username: String::from ("doe@mail.com"),
        password: String::from ("12345")
    };

    let mut pay_with_card = Payment::CreditCard(String::from("540-60-12"));
    let pay_debit = Payment::DebitCard(credents);

     println!("{:?}\n{:?}", pay_with_card, pay_debit);



}