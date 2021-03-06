use crate::{Board, PieceMove, PiecePosition};

pub trait GetAvailableMoves<T: serde::Serialize> {
    fn get_available_moves(
        &self,
        pos: impl Into<PiecePosition>,
        board: &Board<T>,
    ) -> Vec<PieceMove>;
}
