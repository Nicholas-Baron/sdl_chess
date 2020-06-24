use chess::{Board, BoardStatus, ChessMove, Color, MoveGen};

use std::time::Instant;

const AI_SIDE: Color = Color::Black;

const MAX_DEPTH: usize = 5;

pub fn best_move(board: &Board) -> ChessMove {
    assert_eq!(board.side_to_move(), chess::Color::Black);
    let start_compute = Instant::now();
    let moves: Vec<_> = MoveGen::new_legal(board)
        .map(|chess_move| {
            println!("Analyzing move {}", chess_move);
            let start = Instant::now();
            let score = alpha_beta(
                board.make_move_new(chess_move),
                MAX_DEPTH,
                isize::MIN,
                isize::MAX,
                true,
            );
            println!("{:?}", Instant::now().duration_since(start));
            (score, chess_move)
        })
        .collect();

    println!(
        "Took {:?} to compute all moves",
        Instant::now().duration_since(start_compute)
    );
    moves.iter().max_by_key(|(score, _)| score).unwrap().1
}

fn score_for(board: Board) -> isize {
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

    let player_count = isize::try_from(
        pieces_on_board
            .iter()
            .filter(|(_, color)| *color != AI_SIDE)
            .count(),
    )
    .unwrap();

    isize::try_from(possible_move_count + ai_count).unwrap() - player_count
}

fn alpha_beta(
    board: Board,
    depth: usize,
    mut alpha: isize,
    mut beta: isize,
    maximize: bool,
) -> isize {
    if depth == 0 || board.status() != BoardStatus::Ongoing {
        return match board.status() {
            BoardStatus::Stalemate => 0,
            BoardStatus::Checkmate => {
                let attacking_square = board.checkers().to_square();
                let attacking_color = board.color_on(attacking_square).unwrap();
                if attacking_color == AI_SIDE {
                    isize::MAX
                } else {
                    isize::MIN
                }
            }
            BoardStatus::Ongoing => score_for(board),
        };
    }

    let moves = MoveGen::new_legal(&board);

    if maximize {
        let mut value = isize::MIN;
        for child in moves.map(|chess_move| board.make_move_new(chess_move)) {
            value = value.max(alpha_beta(child, depth - 1, alpha, beta, false));
            alpha = alpha.max(value);
            if alpha >= beta {
                return value;
            }
        }
        value
    } else {
        let mut value = isize::MAX;
        for child in moves.map(|chess_move| board.make_move_new(chess_move)) {
            value = value.min(alpha_beta(child, depth - 1, alpha, beta, true));
            beta = beta.min(value);
            if beta <= alpha {
                return value;
            }
        }
        value
    }
}
