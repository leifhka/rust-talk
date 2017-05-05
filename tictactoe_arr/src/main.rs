extern crate rand;

use std::io;
use std::fmt;
use std::option::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mark {
	O,
	X,
	E,
}

#[derive(Debug)]
struct Game {
	board: [Mark; 9],
	turn: Mark,
	winner: Mark,
}

impl fmt::Display for Mark {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let printable = match *self {
			Mark::O => "O",
			Mark::X => "X",
			_ => " ",
		};
		write!(f, "{}", printable)
	}
}

impl fmt::Display for Game {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = String::from("");
		let mut b = String::from("");

		for i in 0..3 {
			let ind = i*3;
			s = s + format!("{}\n│{}│{}│{}│\n", b, self.board[ind], self.board[ind+1], self.board[ind+2]).as_str();
			b = String::from("├─┼─┼─┤");
		}

		write!(f, "┌─┬─┬─┐{}└─┴─┴─┘", s)
	}
}

impl Mark {
	fn is_empty(&self) -> bool {
		match *self {
			Mark::E => true,
			_ => false,
		}
	}
}

impl Game {
	fn new() -> Self {
		use Mark::*;
		Game { board: [E; 9], turn: X, winner: E }
	}

	fn flip(&self) -> Game {
		let mut flipped = [Mark::E; 9];
		for i in 0..3 {
			for j in 0..3 {
				flipped[3*i + j] = self.board[3*j+i];
			}
		}
		return Game {board: flipped, winner: self.winner, turn: self.turn};
	}

	fn is_won_horizontal(&self) -> Option<Mark> {
		let b = &self.board;

		if !b[0].is_empty() && ((b[0] == b[1] && b[1] == b[2]) || (b[0] == b[4] && b[4] == b[8])) {
			Some(b[0])
		} else if !b[3].is_empty() && b[3] == b[4] && b[4] == b[5] {
			Some(b[3])
		} else if !b[6].is_empty() && b[6] == b[7] && b[7] == b[8] {
			Some(b[6])
		} else {
			None
		}
	}

	fn flip_turn(&mut self) {
		match self.turn {
			Mark::X => self.turn = Mark::O,
			_ => self.turn = Mark::X,
		}
	}

	fn is_won(&mut self) -> bool {
		if let Some(winner) = self.is_won_horizontal().or(self.flip().is_won_horizontal()) {
			self.winner = winner;
			true
		} else {
			false
		}
	}

	fn is_done(&mut self) -> bool {
		self.is_won() || self.is_tie()
	}

	fn is_tie(&self) -> bool {
		!self.board.iter().any(|x| (*x == Mark::E))
	}

	fn make_move(&mut self, m: Mark, i: usize) -> bool {
		if self.board[i] == Mark::E {
			self.board[i] = m;
			self.flip_turn();
			true
		} else {
			false
		}
	}

	fn player_move(&mut self) -> bool {
		match self.turn {
			Mark::X => self.make_player_move(),
			_ => self.make_ai_move(),
		}
	}

	fn make_player_move(&mut self) -> bool {
		let mut input_text = String::new();
		io::stdin()
			.read_line(&mut input_text)
			.expect("failed to read from stdin");
		let trimmed = input_text.trim();
		if let Ok(i) = trimmed.parse::<usize>() {
			self.make_move(Mark::X, i)
		} else {
			false
		}
	}

	fn make_ai_move(&mut self) -> bool {
		use rand::distributions::{IndependentSample, Range};

		let mut rng = rand::thread_rng();
		let between = Range::new(0, 9);
		self.make_move(Mark::O, between.ind_sample(&mut rng))
	}
}

fn print_loop_text(b: &Game) {
	println!("{}\nMake a move: ", &b);
}

fn main() {

	let mut game = Game::new();
	while !game.is_done() {
		print_loop_text(&game);
		while !game.player_move() {}
	}
	println!("{}", &game);
	match game.winner {
		Mark::E => println!("Tie!"),
		_ => println!("Player {} won!", game.winner),
	}
}
