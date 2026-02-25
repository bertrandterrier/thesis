pub enum Tile {
    Back(Box<Tile>),
    Std()
}

pub struct TileData {
}
