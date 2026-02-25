use std::collections::VecDeque;
use crate::pcs::tiles::Tile;

pub enum MvDir {
    Null,
    Xy(i16, i16),
    SetFree(i128, i128),
    Horiz(i16),
    Vert(i16),
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
    UpLeft(usize),
    UpRight(usize),
    DownLeft(usize),
    DownRight(usize),
    Center,
}

pub struct Point {
    x: i128,
    y: i128
}

impl Point {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    fn x_move(&mut self, i: i128) {
        self.x += i;
    }
    pub fn mv(&self, movement: MvDir) -> Self {
        Self::from(match &movement {
            MvDir::Null => (self.x, self.y),
            MvDir::SetFree(x, y) => (*x, *y),
            MvDir::Center => (0, 0),
            MvDir::Xy(x, y) => (self.x + *x as i128, self.y + *y as i128),
            _ => (
                match &movement {
                    MvDir::Left(l) | MvDir::UpLeft(l) | MvDir::DownLeft(l) => -(*l as i128),
                    MvDir::Right(r) | MvDir::UpRight(r) | MvDir::DownRight(r) => *r as i128,
                    MvDir::Vert(x) => *x as i128,
                    _ => 0
                },
                match &movement {
                    MvDir::Vert(y) => *y as i128,
                    MvDir::Up(u) | MvDir::UpLeft(u) | MvDir::UpRight(u) => *u as i128,
                    MvDir::Down(d) | MvDir::DownRight(d) | MvDir::DownLeft(d) => -(*d as i128),
                    _ => 0
                }
            )
        })
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Self::from((self.x, self.y))
    }
}

impl<T: Into<i128>> From<(T, T)> for Point {
    fn from(src: (T, T)) -> Self {
        Self { x: src.0.into(), y: src.1.into() }
    }
}


pub struct Field {
    pos: Point,
    pub tiles: Option<TileState>,
}

impl Field {
    pub fn new<T: Into<i128>>(x: T, y: T) -> Self {
        Self {
            pos: Point::from((x, y)),
            tiles: vec![],
        }
    }
    pub fn place(&mut self, force: bool, t: Tile) {
    } 
}
