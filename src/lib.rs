use rand::Rng;

pub struct Card {
    suit: &'static str,
    value: u8,
}

impl Card {
    pub fn new(suit: &'static str, value: u8) -> Self {
        Self {
            suit,
            value,
        }
    }

    pub fn suit(&self) -> &'static str {
        self.suit
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn print(&self) {
        match self.value {
            13=> println!("King of {}", self.suit),
            12=> println!("Queen of {}", self.suit),
            11=> println!("Jack of {}", self.suit),
            1=> println!("Ace of {}", self.suit),
            _=> println!("{} of {}", self.value, self.suit),
        }
    }

}

pub struct Deck {
    deck: Vec<Card>,
    size: u8,
}

impl Deck {
    pub fn new() -> Self {
        let mut d = Self {
            deck: Vec::new(),
            size: 0,
        };
        let suits = &["Spades", "Clubs", "Hearts", "Diamonds"];
        let range = 13;
        for suit in suits {
            for val in 1..=range {
                d.deck.push(Card::new(suit, val));
                d.size += 1;
            }
        }
        return d;
    }
    
    pub fn deck(&self) -> &Vec<Card> {
        &self.deck
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    pub fn deal(&mut self) -> Card {
        let card_pos: usize = rand::thread_rng().gen_range(0..self.size).into();
        let card = self.deck.remove(card_pos);
        self.size -= 1;
        return card;
    }
}