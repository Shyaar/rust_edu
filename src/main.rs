
#[derive(Debug)]
#[allow(unused,unused_variables)]
enum CardSuit {
    Hearts(String),
    Diamonds(String),
    Spades(String),
    Clubs(String),
}

struct Card {
    rank: String,
    suit: CardSuit,
}
fn main(){
    let firstCard= CardSuit::Hearts(String::from ("10"));


    println!("{:?}", firstCard);
    
}