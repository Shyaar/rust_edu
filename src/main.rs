
#[derive(Debug)]
#[allow(unused,unused_variables)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

struct Card {
    rank: String,
    suit: CardSuit,
}
fn main(){
    let firstCard = CardSuit::Hearts;
    let mut secondcard=CardSuit::Spades;
    let secondhad = CardSuit::Clubs;

    let suits = [CardSuit::Hearts, CardSuit::Clubs];

}