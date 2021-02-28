use chess::{Board, BoardStatus, ChessMove, Color, MoveGen, Piece};

use rayon::prelude::*;

use std::collections::VecDeque;
use std::time::Instant;

pub const AI_SIDE: Color = Color::Black;

type ScoreType = isize;

const MAX_DEPTH: usize = 6;

const fn min_score() -> ScoreType {
    ScoreType::MIN
}

const fn max_score() -> ScoreType {
    ScoreType::MAX
}

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

    fn into_children(mut self) -> AIChildren {
        self.children
            .take()
            .unwrap_or_else(|| Self::compute_children(&self.board))
    }

    fn find(&mut self, board: Board) -> Option<Self> {
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

    fn alpha_beta(
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

const fn points_for_piece(piece: Piece) -> ScoreType {
    match piece {
        Piece::Pawn => 1,
        Piece::Knight | Piece::Bishop => 3,
        Piece::Rook => 5,
        Piece::Queen => 9,
        Piece::King => 200,
    }
}

pub fn best_move(board: Board, mut ai_state: AIState) -> (ChessMove, AIState) {
    assert_eq!(board.side_to_move(), chess::Color::Black);
    let start_compute = Instant::now();

    let current_state = ai_state.find(board).unwrap();
    // We can strip away all the information about the given AI State, as we will not need it.
    let children = current_state.into_children();

    assert!(!children.is_empty());

    // If there is a single move, no need to compute the best possible move
    if children.len() == 1 {
        return children.first().unwrap().clone();
    }

    let moves: Vec<_> = children
        .into_par_iter()
        .map(|(chess_move, mut ai_state)| {
            let start = Instant::now();
            let (score, ai_state) = ai_state.alpha_beta(MAX_DEPTH, min_score(), max_score());
            println!(
                "Analysis of {} took {:?}",
                chess_move,
                Instant::now().duration_since(start)
            );
            (score, chess_move, ai_state.clone())
        })
        .collect();

    println!(
        "Took {:?} to compute all moves",
        Instant::now().duration_since(start_compute)
    );

    let (_score, ai_move, ai_state) = moves
        .into_iter()
        .max_by_key(|(score, _, _)| *score)
        .unwrap();

    (ai_move, ai_state)
}

fn score_for(board: Board) -> ScoreType {
    // First, count the number of possible moves for the AI (maximize)
    let possible_move_count = MoveGen::new_legal(&board)
        .filter_map(|chess_move| {
            let src_square = chess_move.get_source();
            board.color_on(src_square)
        })
        .filter(|&color| color == AI_SIDE)
        .count();

    // Then, count the AI and player's points
    // Maximize AI points, minimize player points
    let pieces_on_board: Vec<_> = chess::ALL_SQUARES
        .iter()
        .filter_map(|square| {
            board
                .piece_on(*square)
                .map(|piece| (piece, board.color_on(*square).unwrap()))
        })
        .collect();

    let ai_points: ScoreType = pieces_on_board
        .iter()
        .filter(|(_, color)| *color == AI_SIDE)
        .map(|(piece, _)| points_for_piece(*piece))
        .sum();

    let player_points: ScoreType = pieces_on_board
        .iter()
        .filter(|(_, color)| *color != AI_SIDE)
        .map(|(piece, _)| points_for_piece(*piece))
        .sum();

    use std::convert::TryFrom;
    ScoreType::try_from(possible_move_count).unwrap() + ai_points - player_points
}
