// Tic-Tac-Toe game in Rust

use colored::Colorize;
use core::fmt::{Display, Formatter};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    X,
    O,
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let symbol = match self {
            Self::X => 'X',
            Self::O => 'O',
            Self::Empty => ' ',
        };
        write!(f, "{symbol}")
    }
}

#[derive(PartialEq)]
struct Row {
    tiles: [Tile; 3],
}

impl Row {
    const fn new() -> Self {
        Self {
            tiles: [Tile::Empty; 3],
        }
    }
}

struct Diagonal<'a> {
    tiles: [&'a Tile; 3],
}

trait Completable {
    fn is_complete(&self, tile: Tile) -> bool;
}

impl Completable for [Tile; 3] {
    fn is_complete(&self, tile: Tile) -> bool {
        self.iter().all(|t| *t == tile)
    }
}

impl Completable for Row {
    fn is_complete(&self, tile: Tile) -> bool {
        self.tiles.is_complete(tile)
    }
}

impl Completable for Diagonal<'_> {
    fn is_complete(&self, tile: Tile) -> bool {
        self.tiles.iter().all(|&t| *t == tile)
    }
}

struct Game {
    board: [Row; 3],
    player: Tile,
    winner: Tile,
    turn: u8,
    over: bool,
}

impl Game {
    pub const fn new() -> Self {
        Self {
            board: [Row::new(), Row::new(), Row::new()],
            player: Tile::X,
            winner: Tile::Empty,
            turn: 0,
            over: false,
        }
    }

    pub fn play(&mut self, index: usize) {
        let row = index / 3;
        let col = index % 3;

        if self.board[row].tiles[col] == Tile::Empty {
            self.board[row].tiles[col] = self.player;
        }
        self.turn += 1;

        if self.is_complete() {
            self.winner = self.player;
            self.over = true;
        } else if self.is_tie() {
            self.over = true;
        } else {
            self.player = match self.player {
                Tile::X => Tile::O,
                Tile::O => Tile::X,
                Tile::Empty => panic!("Invalid player"),
            };
        }
    }

    pub fn is_complete(&self) -> bool {
        self.any_row_complete(Tile::X)
            || self.any_row_complete(Tile::O)
            || self.any_diagonal_complete(Tile::X)
            || self.any_diagonal_complete(Tile::O)
            || self.any_col_complete(Tile::X)
            || self.any_col_complete(Tile::O)
    }

    pub fn any_row_complete(&self, tile: Tile) -> bool {
        self.board.iter().any(|row| row.is_complete(tile))
    }

    pub fn any_col_complete(&self, tile: Tile) -> bool {
        let cols = self.cols();
        cols.iter().any(|col| col.is_complete(tile))
    }

    pub fn any_diagonal_complete(&self, tile: Tile) -> bool {
        let diags = self.diagonals();
        diags.iter().any(|diag| diag.is_complete(tile))
    }

    pub const fn diagonals(&self) -> [Diagonal; 2] {
        [
            Diagonal {
                tiles: [
                    &self.board[0].tiles[0],
                    &self.board[1].tiles[1],
                    &self.board[2].tiles[2],
                ],
            },
            Diagonal {
                tiles: [
                    &self.board[0].tiles[2],
                    &self.board[1].tiles[1],
                    &self.board[2].tiles[0],
                ],
            },
        ]
    }

    pub const fn cols(&self) -> [Row; 3] {
        [
            Row {
                tiles: [
                    self.board[0].tiles[0],
                    self.board[1].tiles[0],
                    self.board[2].tiles[0],
                ],
            },
            Row {
                tiles: [
                    self.board[0].tiles[1],
                    self.board[1].tiles[1],
                    self.board[2].tiles[1],
                ],
            },
            Row {
                tiles: [
                    self.board[0].tiles[2],
                    self.board[1].tiles[2],
                    self.board[2].tiles[2],
                ],
            },
        ]
    }

    pub const fn is_tie(&self) -> bool {
        self.turn == 9
    }

    pub const fn game_over(&self) -> bool {
        self.over
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for (i, row) in (0u32..).zip(self.board.iter()) {
            for (j, tile) in (0u32..).zip(row.tiles.iter()) {
                let symbol = match tile {
                    Tile::X => 'X',
                    Tile::O => 'O',
                    Tile::Empty => char::from_digit(i * 3 + j, 10).unwrap_or(' '),
                };
                match symbol {
                    'X' => write!(f, "{} ", symbol.to_string().green()),
                    'O' => write!(f, "{} ", symbol.to_string().red()),
                    _ => write!(f, "{symbol} "),
                }?;
            }
            writeln!(f)?;
            writeln!(f, "-----")?;
        }
        Ok(())
    }
}

use std::io::Error;

fn main() -> Result<(), Error> {
    let mut game = Game::new();

    while !game.game_over() {
        println!("{game}");
        println!("Player {}, enter your move (0-8):", game.player);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if let Ok(index) = input.trim().parse() {
            game.play(index);
        } else {
            println!("Invalid input, please enter a number between 0 and 8");
            continue;
        }
    }
    println!("{game}");
    match game.winner {
        Tile::X => println!("Player X wins!"),
        Tile::O => println!("Player O wins!"),
        Tile::Empty => println!("It's a tie!"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_is_complete() {
        let row = Row {
            tiles: [Tile::X, Tile::X, Tile::X],
        };
        assert!(row.is_complete(Tile::X));
    }

    #[test]
    fn test_diagonal_is_complete() {
        let diag = Diagonal {
            tiles: [&Tile::X, &Tile::X, &Tile::X],
        };
        assert!(diag.is_complete(Tile::X));
    }

    #[test]
    fn test_game_is_complete() {
        let mut game = Game::new();
        game.play(0);
        game.play(1);
        game.play(3);
        game.play(2);
        game.play(6);
        assert!(game.is_complete());
    }
    #[test]
    fn test_game_is_tie() {
        let mut game = Game::new();
        game.play(0);
        game.play(1);
        game.play(2);
        game.play(3);
        game.play(4);
        game.play(5);
        game.play(6);
        game.play(7);
        game.play(8);
        assert!(game.is_tie());
    }
}
