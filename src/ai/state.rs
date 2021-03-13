use super::{max_score, min_score, score_for, ScoreType, AI_SIDE};

use chess::{Board, BoardStatus, ChessMove, MoveGen};

use dashmap::DashMap as Map;

#[derive(Default)]
pub struct AIState {
    rankings: Map<Board, ScoreType>,
}

impl AIState {
    pub(super) fn children_of(&self, board: &Board) -> Vec<(ChessMove, Board)> {
        MoveGen::new_legal(&board)
            .map(|chess_move| {
                let new_board = board.make_move_new(chess_move);
                if !self.rankings.contains_key(&new_board) {
                    self.rankings.insert(new_board, score_for(&new_board));
                }
                (chess_move, new_board)
            })
            .collect()
    }

    pub(super) fn known_board_states(&self) -> usize {
        self.rankings.len()
    }

    pub(super) fn alpha_beta(
        &self,
        board: Board,
        depth: usize,
        mut alpha: ScoreType,
        mut beta: ScoreType,
    ) -> ScoreType {
        if depth == 0 || board.status() != BoardStatus::Ongoing {
            return match board.status() {
                BoardStatus::Stalemate => 0,
                BoardStatus::Checkmate => {
                    let attacking_square = board.checkers().to_square();
                    let attacking_color = board.color_on(attacking_square).unwrap();
                    if attacking_color == AI_SIDE {
                        max_score()
                    } else {
                        min_score()
                    }
                }
                BoardStatus::Ongoing => *self.rankings.get(&board).unwrap(),
            };
        }

        let maximize = board.side_to_move() == AI_SIDE;
        let mut value = if maximize { min_score() } else { max_score() };

        for (_, child) in self.children_of(&board).into_iter() {
            let next_value = self.alpha_beta(child, depth - 1, alpha, beta);
            self.rankings.insert(child, next_value);
            if maximize {
                if value < next_value {
                    value = next_value;
                    alpha = alpha.max(value);
                    if alpha >= beta {
                        return value;
                    }
                }
            } else if value > next_value {
                value = next_value;
                beta = beta.min(value);
                if beta <= alpha {
                    return value;
                }
            }
        }
        value
    }
}
