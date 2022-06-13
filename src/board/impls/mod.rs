use crate::{GetAvailableMoves, GetPiece, PieceMove, PiecePosition};

pub use super::*;

pub mod get_eval;
pub use get_eval::*;

pub mod print_debug;
pub use print_debug::*;

pub mod default;
pub use default::*;

impl<T: GetPiece + GetAvailableMoves<T> + Copy + serde::Serialize> Board<T> {
    pub fn at<'a>(&'a self, pos: impl Into<PiecePosition>) -> &'a T {
        &self.cells[*pos.into()]
    }

    pub fn at_mut<'a>(&'a mut self, pos: impl Into<PiecePosition>) -> &'a mut T {
        &mut self.cells[*pos.into()]
    }

    pub fn get_available_moves(&self, pos: impl Into<PiecePosition>) -> Vec<PieceMove> {
        let pos = pos.into();
        let cell = &self.cells[*pos];
        cell.get_available_moves(pos, self)
    }
}
