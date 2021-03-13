use chess::{Board, ChessMove, Color, MoveGen, Piece};

use rayon::prelude::*;

use std::time::Instant;

mod state;
pub use state::AIState;

pub const AI_SIDE: Color = Color::Black;

type ScoreType = isize;

const MAX_DEPTH: usize = 6;

const fn min_score() -> ScoreType {
    ScoreType::MIN
}

const fn max_score() -> ScoreType {
    ScoreType::MAX
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

fn children_of(board: &Board) -> Vec<(ChessMove, Board)> {
    MoveGen::new_legal(board)
        .map(|chess_move| (chess_move, board.make_move_new(chess_move)))
        .collect()
}

pub fn best_move(board: Board, ai_state: &AIState) -> ChessMove {
    assert_eq!(board.side_to_move(), AI_SIDE);
    println!("Starting timer");
    let start_compute = Instant::now();

    let children = children_of(&board);
    println!("Got {} children for current board", children.len());

    assert!(!children.is_empty());

    // If there is a single move, no need to compute the best possible move
    if children.len() == 1 {
        return children.first().unwrap().0;
    }

    let moves: Vec<_> = children
        .into_par_iter()
        .map(|(chess_move, child_board)| {
            let start = Instant::now();
            println!("Starting alpha_beta for {}", chess_move);
            let score = ai_state.alpha_beta(child_board, MAX_DEPTH, min_score(), max_score());
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

    println!("{}", ai_state.known_board_states());
    let (_score, ai_move) = moves.into_iter().max_by_key(|(score, _)| *score).unwrap();

    ai_move
}

fn score_for(board: &Board) -> ScoreType {
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
