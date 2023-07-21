use bevy::prelude::Component;

#[derive(Component)]
pub struct ChessTile {
    color: TileColor,
    piece: Occupation,
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
    None,
}