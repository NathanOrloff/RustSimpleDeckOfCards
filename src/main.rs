use cards::{Card, Deck};

fn main() {
    let mut my_deck = Deck::new();
    println!("{}", my_deck.size());
    let card = my_deck.deal();
    card.print();
    println!("{}", my_deck.size());
    
}
