mod solution;
use solution::ChessGame;
use solution::Piece::*;
use solution::PieceType::*;
use solution::Turn;
use solution::Error;

use std::convert::TryInto;


fn main() -> Result<(), Error>{
    let mut game = ChessGame::new_game();
    //println!("{:?}",game);
    //println!("{:?}",game.current_player());

    let dst: solution::Position = "e8".try_into()?;
    let src: solution::Position = "a8".try_into()?;

    let text = match game.get_field(dst) {
        Some(White(Rook)) => "white rook",
        Some(White(Knight)) => "white knight",
        Some(White(Bishop)) => "white bishop",
        Some(White(Queen)) => "white queen",
        Some(White(King)) => "white king",
        Some(White(Pawn)) => "white pawn",
        Some(Black(Rook)) => "black rook",
        Some(Black(Knight)) => "black knight",
        Some(Black(Bishop)) => "black bishop",
        Some(Black(Queen)) => "black queen",
        Some(Black(King)) => "black king",
        Some(Black(Pawn)) => "black pawn",
        None => "empty field",
    };

    let text = match game.current_player() {
        Turn::WhitePlays => "white plays",
        Turn::BlackPlays => "black plays",
    };
    println!("{}",text);
    
    /*
    let text = match game.make_move(src, dst) {
        Ok(None) => "valid move",
        Ok(Some(White(Pawn))) => "valid move & white pawn taken",
        Err(Error::InvalidMove) => "invalid move",
        _ => "other",
    };
    println!("{}",text);

     */
    Ok(())
}