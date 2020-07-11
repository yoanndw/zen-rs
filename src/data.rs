#[derive(Debug)]
pub enum Gamemode {
    Solo,
    Multi,
}

pub enum TransData {
    Mode(Gamemode),
    None,
    // TODO: winner
}
