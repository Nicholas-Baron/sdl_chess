use std::collections::HashMap;
use std::convert::TryFrom;

use chess::{Board, BoardStatus, ChessMove, Color, MoveGen, Piece};

type ScoreType = isize;

const DEPTH: u8 = 4;

const fn points_for_piece(piece: Piece) -> ScoreType {
    match piece {
        Piece::Pawn => 1,
        Piece::Knight | Piece::Bishop => 3,
        Piece::Rook => 5,
        Piece::Queen => 9,
        Piece::King => 200,
    }
}

fn guess_score(player: Color, board: Board) -> ScoreType {
    let mut score = 0;

    // Pieces on board
    for piece in [
        Piece::Pawn,
        Piece::Knight,
        Piece::Bishop,
        Piece::Rook,
        Piece::Queen,
        Piece::King,
    ] {
        let pieces_of_player = board.color_combined(player) & board.pieces(piece);
        let pieces_of_opponent = board.color_combined(!player) & board.pieces(piece);
        score += points_for_piece(piece) * ScoreType::try_from(pieces_of_player.popcnt()).unwrap();
        score -=
            points_for_piece(piece) * ScoreType::try_from(pieces_of_opponent.popcnt()).unwrap();
    }

    // Giving check to king
    let checkers_value = ScoreType::try_from(board.checkers().popcnt()).unwrap() * 10;

    // Pinning something to the king
    let pin_value = ScoreType::try_from(board.pinned().popcnt()).unwrap() * 5;

    if player != board.side_to_move() {
        score += checkers_value + pin_value;
    }

    score
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct CacheEntry {
    chess_move: ChessMove,
    score: ScoreType,
    color: Color,
    eval_depth: u8,
}

impl CacheEntry {
    fn update_if_better(&mut self, new_entry: Self) {
        if *self == new_entry {
            return;
        }

        // Always pick the deeper eval
        match self.eval_depth.cmp(&new_entry.eval_depth) {
            std::cmp::Ordering::Less => {
                *self = new_entry;
                return;
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => return,
        }

        assert_eq!(self.color, new_entry.color);

        if self.score.abs() < new_entry.score.abs() {
            *self = new_entry;
        }
    }
}

#[derive(Default)]
pub struct AIState {
    /// Cache scores and the color that the score is used for
    score_cache: HashMap<String, CacheEntry>,
}

impl AIState {
    pub fn best_move(&mut self, board: Board, player: Color) -> ChessMove {
        let chess_move = self
            .alpha_beta(board, DEPTH, ScoreType::MIN + 1, ScoreType::MAX, player)
            .0
            .unwrap();
        println!("Evaluated {} positions", self.score_cache.len());
        chess_move
    }

    fn alpha_beta(
        &mut self,
        board: Board,
        depth: u8,
        mut alpha: ScoreType,
        beta: ScoreType,
        player: Color,
    ) -> (Option<ChessMove>, ScoreType) {
        if depth == 0 || board.status() != BoardStatus::Ongoing {
            return match (board.status(), self.score_cache.get(&board.to_string())) {
                (BoardStatus::Stalemate, _) => (None, 0),
                (BoardStatus::Checkmate, _) => (None, ScoreType::MIN + 1),
                (BoardStatus::Ongoing, Some(entry)) => (Some(entry.chess_move), entry.score),
                (BoardStatus::Ongoing, None) => (None, guess_score(player, board)),
            };
        }

        let mut moves: Vec<_> = MoveGen::new_legal(&board).collect();

        moves
            .sort_by_cached_key(|chess_move| guess_score(player, board.make_move_new(*chess_move)));

        let mut best_so_far: Option<ChessMove> = None;
        for chess_move in moves {
            let next_board = board.make_move_new(chess_move);
            let score = -self
                .alpha_beta(next_board, depth - 1, -beta, -alpha, !player)
                .1;
            if score >= beta {
                if let Some(best_so_far) = best_so_far {
                    self.update_cache(board.to_string(), best_so_far, beta, player, depth);
                }
                return (best_so_far, beta);
            }
            if score > alpha {
                alpha = score;
                best_so_far = Some(chess_move);
            }
        }

        if let Some(best_so_far) = best_so_far {
            self.update_cache(board.to_string(), best_so_far, alpha, player, depth);
        }
        (best_so_far, alpha)
    }

    fn update_cache(
        &mut self,
        board_string: String,
        chess_move: ChessMove,
        score: ScoreType,
        player: Color,
        eval_depth: u8,
    ) {
        let new_cache_entry = CacheEntry {
            score,
            color: player,
            eval_depth,
            chess_move,
        };

        self.score_cache
            .entry(board_string)
            .and_modify(|entry| entry.update_if_better(new_cache_entry.clone()))
            .or_insert(new_cache_entry);
    }
}
