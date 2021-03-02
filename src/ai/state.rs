use super::{max_score, min_score, score_for, ScoreType, AI_SIDE};

use chess::{Board, BoardStatus, ChessMove, MoveGen};

use std::collections::VecDeque;

type AIChildren = Vec<(ChessMove, AIState)>;

#[derive(Clone)]
pub struct AIState {
    board: Board,
    score: ScoreType,
    // `None` signifies no computations done.
    // Empty `Vec` signifies an end of game.
    children: Option<AIChildren>,
}

impl AIState {
    pub fn analyze_board(board: Board) -> Self {
        Self {
            board,
            score: score_for(board),
            children: Some(Self::compute_children(&board)),
        }
    }

    fn compute_children(board: &Board) -> Vec<(ChessMove, AIState)> {
        MoveGen::new_legal(&board)
            .map(|chess_move| {
                (
                    chess_move,
                    Self::lazy_convert(board.make_move_new(chess_move)),
                )
            })
            .collect()
    }

    fn lazy_convert(board: Board) -> Self {
        Self {
            board,
            score: score_for(board),
            children: None,
        }
    }

    fn children(&mut self) -> &mut AIChildren {
        let board = self.board;
        self.children
            .get_or_insert_with(|| Self::compute_children(&board))
    }

    pub(super) fn into_children(mut self) -> AIChildren {
        self.children
            .take()
            .unwrap_or_else(|| Self::compute_children(&self.board))
    }

    pub(super) fn find(&mut self, board: Board) -> Option<Self> {
        let mut to_search: VecDeque<AIState> = VecDeque::new();
        for (_, child) in self.children() {
            to_search.push_back(child.clone());
        }

        while let Some(mut child) = to_search.pop_front() {
            if child.board == board {
                return Some(child);
            }
            for (_, child) in child.children() {
                to_search.push_back(child.clone());
            }
        }
        None
    }

    pub(super) fn alpha_beta(
        &mut self,
        depth: usize,
        mut alpha: ScoreType,
        mut beta: ScoreType,
    ) -> (ScoreType, &mut AIState) {
        if depth == 0 || self.board.status() != BoardStatus::Ongoing {
            return (
                match self.board.status() {
                    BoardStatus::Stalemate => 0,
                    BoardStatus::Checkmate => {
                        let attacking_square = self.board.checkers().to_square();
                        let attacking_color = self.board.color_on(attacking_square).unwrap();
                        if attacking_color == AI_SIDE {
                            max_score()
                        } else {
                            min_score()
                        }
                    }
                    BoardStatus::Ongoing => score_for(self.board),
                },
                self,
            );
        }

        let maximize = self.board.side_to_move() == AI_SIDE;
        let mut value = if maximize { min_score() } else { max_score() };
        let mut result_state = None;

        for (_, child) in self.children().iter_mut() {
            let (next_value, ai_state) = child.alpha_beta(depth - 1, alpha, beta);
            if maximize {
                if value < next_value {
                    value = next_value;
                    alpha = alpha.max(value);
                    if alpha >= beta {
                        return (value, ai_state);
                    }
                    result_state = Some(ai_state);
                }
            } else if value > next_value {
                value = next_value;
                beta = beta.min(value);
                if beta <= alpha {
                    return (value, ai_state);
                }
                result_state = Some(ai_state);
            }
        }
        (value, result_state.unwrap())
    }
}
