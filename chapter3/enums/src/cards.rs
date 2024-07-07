
#[allow(dead_code)]
enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[allow(dead_code)]
enum Card {
    King(Suit),
    Queen(Suit),
    Jack(Suit),
    Ace(Suit),
    Pip(Suit, usize),
}
