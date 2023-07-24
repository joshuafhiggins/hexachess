use bevy::{prelude::Component, render::color};

#[derive(Component)]
pub struct ChessTile {
    color: TileColor,
    piece: Occupation,
}

impl ChessTile {
    pub fn new(color: TileColor, piece: Occupation) -> ChessTile {
        ChessTile { color: color, piece: piece }
    }
}

pub enum TileColor {
    BLACK,
    GREY,
    WHITE,
}

pub enum Team {
    WHITE,
    BLACK,
}

pub enum Occupation {
    PAWN(Team),
    KNIGHT(Team),
    BISHOP(Team),
    ROOK(Team),
    QUEEN(Team),
    KING(Team),
    NONE,
}