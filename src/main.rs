
#[derive(Debug)]
#[allow(unused,unused_variables)]
enum Payment {
    CreditCard(String),
    DebitCard(String, String),
    Wallet(String),
}
fn main(){
    let mut pay_with_card = Payment::CreditCard(String::from("540-60-12"));
     pay_with_card = Payment::DebitCard(String::from("James"), String::from("231-099-890"));

     println!("{:?}", pay_with_card)



}