#[derive(Debug)]
#[allow(unused, unused_variables)]

// struct Credentials {
//     username: String,
//     password: String,
// }
// #[derive(Debug)]

// enum Payment {
//     CreditCard(String),
//     DebitCard(Credentials),
//     Wallet { address: String, user_id: u32 },
// }

// impl Payment{

//     fn new(address:String, user_id:u32)->Self{
//         Self::Wallet { address, user_id }
//     }
// }
// fn main() {
//     let credents = Credentials {
//         username: String::from("doe@mail.com"),
//         password: String::from("12345"),
//     };

//     let mut pay_with_card = Payment::CreditCard(String::from("540-60-12"));
//     let pay_debit = Payment::DebitCard(credents);

//     let use_wallet = Payment::Wallet {
//         address: String::from("0x1111"),
//         user_id: (1),
//     };

//     let user2 = Payment::Wallet.new();

//     println!("{:?}\n{:?}\n{:#?}", pay_with_card, pay_debit, use_wallet);
// }


enum Veggies {
    Onions,
    Tomatoe,
}
#[derive(Debug)]

enum Meat {
    Chiken,
    Beef,
}

#[derive(Debug)]

enum Meal {
    Sharawma{meat: Meat, veggie: Veggies},
    SandWich{meat: Meat, veggie: Veggies},
    Stew,
}

fn main(){
    let order = Meal::SandWich{meat: Meat::Beef, veggie:Veggies::Tomatoe};
    let order2= Meal::Sharawma{meat:Meat::Chiken, veggie:Veggies::Tomatoe};

    println!("order 1 :::: {:#?}", order);
    println!("Order 2 >>>> {:#?}", order2);
}