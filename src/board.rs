pub enum Team {
    Black,
    White,
}

pub enum Pawn {
    Player(Team),
    Zen,
}

pub struct Square {
    pawn: Option<Pawn>,
}
