use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum Team {
    Black,
    White,
}

impl Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Team::Black => "b".to_owned(),
                Team::White => "w".to_owned(),
            }
        )
    }
}

pub enum Pawn {
    Player(Team),
    Zen,
}

impl Display for Pawn {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Pawn::Player(t) => t.to_string(),
                Pawn::Zen => "z".to_owned(),
            }
        )
    }
}

pub struct Square {
    pawn: Option<Pawn>,
}

impl Square {
    pub fn without_pawn() -> Self {
        Self { pawn: None }
    }

    pub fn with_team(team: Team) -> Self {
        Self {
            pawn: Some(Pawn::Player(team)),
        }
    }

    pub fn with_zen() -> Self {
        Self {
            pawn: Some(Pawn::Zen),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match &self.pawn {
                Some(p) => p.to_string(),
                None => ".".to_owned(),
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_team() {
        assert_eq!(format!("{}", Team::Black), "b".to_owned());
        assert_eq!(format!("{}", Team::White), "w".to_owned());
    }

    #[test]
    fn print_pawn() {
        assert_eq!(format!("{}", Pawn::Player(Team::Black)), "b".to_owned());
        assert_eq!(format!("{}", Pawn::Player(Team::White)), "w".to_owned());
        assert_eq!(format!("{}", Pawn::Zen), "z".to_owned());
    }

    #[test]
    fn print_square() {
        assert_eq!(
            format!(
                "{}",
                Square {
                    pawn: Some(Pawn::Player(Team::Black))
                }
            ),
            "b".to_owned()
        );

        assert_eq!(
            format!(
                "{}",
                Square {
                    pawn: Some(Pawn::Player(Team::White))
                }
            ),
            "w".to_owned()
        );

        assert_eq!(
            format!(
                "{}",
                Square {
                    pawn: Some(Pawn::Zen)
                }
            ),
            "z".to_owned()
        );

        assert_eq!(format!("{}", Square { pawn: None }), ".".to_owned());
    }
}

pub struct Board {
    data: [[Square; 11]; 11],
}

impl Board {
    pub fn new() -> Self {
        use Team::*;

        let data = [
            [
                Square::with_team(White),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(Black),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(Black),
            ],
            [
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(White),
                Square::without_pawn(),
                Square::with_team(White),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
            ],
            [
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(Black),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(Black),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
            ],
            [
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(White),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(White),
                Square::without_pawn(),
                Square::without_pawn(),
            ],
            [
                Square::without_pawn(),
                Square::with_team(Black),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(Black),
                Square::without_pawn(),
            ],
            [
                Square::with_team(White),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_zen(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(White),
            ],
            [
                Square::without_pawn(),
                Square::with_team(Black),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(Black),
                Square::without_pawn(),
            ],
            [
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(White),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(White),
                Square::without_pawn(),
                Square::without_pawn(),
            ],
            [
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(Black),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(Black),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
            ],
            [
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(White),
                Square::without_pawn(),
                Square::with_team(White),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
            ],
            [
                Square::with_team(Black),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(Black),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::without_pawn(),
                Square::with_team(White),
            ],
        ];

        Self { data }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = self
            .data
            .iter()
            .map(|l| l.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(""))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", s)
    }
}
