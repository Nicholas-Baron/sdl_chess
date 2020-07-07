use chess::{Board, BoardStatus, ChessMove, Color, MoveGen};

use rayon::prelude::*;

use std::time::Instant;

pub const AI_SIDE: Color = Color::Black;

type ScoreType = isize;

const MAX_DEPTH: usize = 6;

fn min_score() -> ScoreType {
    ScoreType::MIN
}

fn max_score() -> ScoreType {
    ScoreType::MAX
}

pub fn best_move(board: &Board) -> ChessMove {
    assert_eq!(board.side_to_move(), chess::Color::Black);
    let start_compute = Instant::now();

    let moves: Vec<_> = MoveGen::new_legal(board).collect();

    assert!(!moves.is_empty());

    // If there is a single move, no need to compute the best possible move
    if moves.len() == 1 {
        return *moves.first().unwrap();
    }

    let moves: Vec<(_, _)> = moves
        .par_iter()
        .map(|chess_move| {
            let start = Instant::now();
            let score = alpha_beta(
                board.make_move_new(*chess_move),
                MAX_DEPTH,
                min_score(),
                max_score(),
                true,
            );
            println!(
                "Analysis of {} took {:?}",
                chess_move,
                Instant::now().duration_since(start)
            );
            (score, chess_move)
        })
        .collect();

    println!(
        "Took {:?} to compute all moves",
        Instant::now().duration_since(start_compute)
    );

    *moves.iter().max_by_key(|(score, _)| score).unwrap().1
}

fn score_for(board: Board) -> ScoreType {
    // First, count the number of possible moves
    let possible_move_count = MoveGen::new_legal(&board).count();

    // Then, count the number of AI and player pieces

    let pieces_on_board: Vec<_> = chess::ALL_SQUARES
        .iter()
        .filter_map(|square| {
            board
                .piece_on(*square)
                .map(|piece| (piece, board.color_on(*square).unwrap()))
        })
        .collect();

    let ai_count = pieces_on_board
        .iter()
        .filter(|(_, color)| *color == AI_SIDE)
        .count();

    use std::convert::TryFrom;

    let player_count = ScoreType::try_from(
        pieces_on_board
            .iter()
            .filter(|(_, color)| *color != AI_SIDE)
            .count(),
    )
    .unwrap();

    ScoreType::try_from(possible_move_count + ai_count).unwrap() - player_count
}

fn alpha_beta(
    board: Board,
    depth: usize,
    mut alpha: ScoreType,
    mut beta: ScoreType,
    maximize: bool,
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
            BoardStatus::Ongoing => score_for(board),
        };
    }

    let moves = MoveGen::new_legal(&board);
    let mut value = if maximize { min_score() } else { max_score() };

    for child in moves.map(|chess_move| board.make_move_new(chess_move)) {
        let next_value = alpha_beta(child, depth - 1, alpha, beta, !maximize);
        if maximize {
            value = value.max(next_value);
            alpha = alpha.max(value);
            if alpha >= beta {
                return value;
            }
        } else {
            value = value.min(next_value);
            beta = beta.min(value);
            if beta <= alpha {
                return value;
            }
        }
    }
    value
}
