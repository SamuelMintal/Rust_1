use core::convert::TryFrom;
use core::convert::TryInto;

#[derive(Debug, Copy, Clone)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Debug, Copy, Clone)]
pub enum Piece {
    White(PieceType),
    Black(PieceType),
}
#[derive(Debug)]
pub enum Error {
    InvalidMove,
    InvalidPosition,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Turn {
    WhitePlays,
    BlackPlays,
}

#[derive(Debug, Clone, Copy)]
pub struct ChessGame {
    fields: [[Option<Piece>; 8]; 8], //fields[y][x]
    on_turn: Turn,    
}

impl ChessGame {
    
    pub fn new_game() -> ChessGame {
        //Todo - place pieces on the board
        let mut tmp_board = [[None; 8]; 8];
        ChessGame::place_all_pieces(&mut tmp_board);        

        ChessGame {
            fields: tmp_board,
            on_turn: Turn::WhitePlays,
        }
    } 

    fn place_all_pieces(board: &mut [[Option<Piece>; 8]; 8]) {

        ChessGame::place_white_pieces(board);
        ChessGame::place_black_pieces(board);
    }

    fn place_white_pieces(board: &mut [[Option<Piece>; 8]; 8]) {

        //Place white Pawns
        for x in 0..8 {                            
            board[1][x] = Some(Piece::White(PieceType::Pawn))
        }

        //Place white rooks
        board[0][0] = Some(Piece::White(PieceType::Rook));
        board[0][7] = Some(Piece::White(PieceType::Rook));
       
        //Place white knights
        board[0][1] = Some(Piece::White(PieceType::Knight));
        board[0][6] = Some(Piece::White(PieceType::Knight));

        //Place white bishops
        board[0][2] = Some(Piece::White(PieceType::Bishop));
        board[0][5] = Some(Piece::White(PieceType::Bishop));

        //Place white king queen
        board[0][4] = Some(Piece::White(PieceType::King));
        board[0][3] = Some(Piece::White(PieceType::Queen));
    }

    fn place_black_pieces(board: &mut [[Option<Piece>; 8]; 8]) {
    
        for x in 0..8 {                            
            board[6][x] = Some(Piece::Black(PieceType::Pawn))
        }

        board[7][0] = Some(Piece::Black(PieceType::Rook));
        board[7][7] = Some(Piece::Black(PieceType::Rook));

        board[7][1] = Some(Piece::Black(PieceType::Knight));
        board[7][6] = Some(Piece::Black(PieceType::Knight));

        board[7][2] = Some(Piece::Black(PieceType::Bishop));
        board[7][5] = Some(Piece::Black(PieceType::Bishop));


        board[7][3] = Some(Piece::Black(PieceType::Queen));
        board[7][4] = Some(Piece::Black(PieceType::King));
    }
    
    pub fn current_player(self) -> Turn {
        self.on_turn
    }

    fn switch_player_on_turn(&mut self) {
               
        if self.on_turn == Turn::WhitePlays {
            self.on_turn = Turn::BlackPlays;
        }
        else {
            self.on_turn = Turn::WhitePlays;
        }
    }

    pub fn make_move(&mut self, src: Position, dst: Position) -> Result<Option<Piece>, Error> {

        let src_piece = self.fields[src.y][src.x];
        let (src_color, src_piece)= match src_piece {
            None => { return Err(Error::InvalidMove); }
            Some(Piece::White(piece)) => ("white", piece),
            Some(Piece::Black(piece)) => ("black", piece),
        };
        //println!("src_color = {:?}", src_color);
        //println!("src_piece = {:?}", src_piece);

        //Checking if src piece is color of player who is on turn
        if src_color == "white" && self.on_turn == Turn::BlackPlays ||
           src_color == "black" && self.on_turn == Turn::WhitePlays  {
            return Err(Error::InvalidMove);
        }

        let dst_piece = self.fields[dst.y][dst.x];
        let (dst_color, dst_piece)= match dst_piece {
            None => ("none", PieceType::Pawn), //Pawn is just a useless placeholder
            Some(Piece::White(piece)) => ("white", piece),
            Some(Piece::Black(piece)) => ("black", piece),
        };

        //println!("dst_color = {:?}", dst_color);
        //println!("dst_piece = {:?}", dst_piece);
        if dst_color == "none" {
            //Move the figure from src to dest
            let src_fig = self.fields[src.y][src.x];
            self.fields[src.y][src.x] = None;
            self.fields[dst.y][dst.x] = src_fig;

            self.switch_player_on_turn();
            return Ok(None);
        }
        else { //dst is position of Piece
            if(src_color == dst_color) {
                return Err(Error::InvalidMove);
            }

            let dead_fig = self.fields[dst.y][dst.x]; 

            let src_fig = self.fields[src.y][src.x];
            self.fields[src.y][src.x] = None;
            self.fields[dst.y][dst.x] = src_fig;
            self.switch_player_on_turn();

            return Ok(dead_fig);
        }
    }

    pub fn get_field(&self, pos: Position) -> Option<Piece> {
        return self.fields[pos.y][pos.x];
    }

}



#[derive(Debug, Clone, Copy)]
pub struct Position {
    y: usize,
    x: usize,
}

impl TryFrom<&str> for Position {
    type Error = Error;

    fn try_from(str_pos: &str) -> Result<Position, Self::Error> {

        let str_len = str_pos.len();
        if str_len != 2 {
            return Err(Error::InvalidPosition);
        }
       
        
        let x_char: char = str_pos.chars().nth(0).unwrap();
        let y_char: char = str_pos.chars().nth(1).unwrap();
        //println!("x_char is {}", x_char);
        //println!("y_char is {}", y_char);
        
        let x_int:i32 = (x_char as i32) - 96;        
        let y_int:i32 = (y_char as i32) - 48;        
        //println!("x_int is {}", x_int);
        //println!("y_int is {}", y_int);
        
        let x_index = (x_int - 1) as usize;
        let y_index = (y_int - 1) as usize;
        //println!("x_index is {}", x_index);
        //println!("y_index is {}", y_index);

        if x_index >= 0 && x_index <= 7 && y_index >= 0 && y_index <= 7 {
            Ok(Position { y: y_index, x: x_index })
        } else {
            Err(Error::InvalidPosition)
        }
    }
}