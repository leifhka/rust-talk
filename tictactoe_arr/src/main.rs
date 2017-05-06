extern crate rand;

use std::io;
use std::fmt;
use std::option::*;

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum RefList<'a> {
	Nil,
	Cons(String, &'a RefList<'a>),
}

#[derive(Debug, Clone)]
enum BoxList {
	Nil,
	Cons(String, Box<BoxList>),
}

#[derive(Clone, Copy, PartialEq)]
enum Mark {
	O,
	X,
	E,
}

struct Game {
	board: [Mark; 9],
	turn: Mark,
	winner: Mark,
}

impl fmt::Display for BoxList {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let printable = match *self {
			BoxList::Cons(ref game, ref l) => String::from(format!("{}\n{}", game, l)),
			_ => String::from(""),
		};
		write!(f, "{}", printable)
	}
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

impl Game {
	fn new() -> Self {
		use Mark::*;
		Game { board: [E; 9], turn: X, winner: E }
	}

	fn flip(&self) -> Game {
		let mut flipped = [Mark::E; 9];
		for i in 0..3 {
			for j in 0..3 {
				flipped[3*i + (2-j)] = self.board[3*j+i];
			}
		}
		return Game {board: flipped, winner: self.winner, turn: self.turn};
	}

	fn is_won_horizontal(&self) -> Option<Mark> {
		let b = &self.board;

		if b[0] != Mark::E && ((b[0] == b[1] && b[1] == b[2]) || (b[0] == b[4] && b[4] == b[8])) {
			Some(b[0])
		} else if b[3] != Mark::E && b[3] == b[4] && b[4] == b[5] {
			Some(b[3])
		} else if b[6] != Mark::E && b[6] == b[7] && b[7] == b[8] {
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

	fn is_tie(&self) -> bool {
		!self.board.iter().any(|x| (*x == Mark::E))
	}

	fn is_done(&mut self) -> bool {
		self.is_won() || self.is_tie()
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
		println!("{}", &self);
		println!("Make a move:");
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

	fn store_win(self) -> String {
		format!("{}", self)
	}
}

fn main() {
	//use RefList::*;
	//let mut refLst = Nil;
	use BoxList::*;
	let mut box_lst = Nil;
	for _ in 0..3 {
    	let mut game = Game::new();
    	while !game.is_done() {
			while !game.player_move() {}
		}
		println!("============\n{}", &game);
		match game.winner {
			Mark::E => println!("Tie!"),
    		_ => println!("Player {} won!", game.winner),
    	}
    	//refLst = Cons(game.store_win(), &lst.clone()); // Results in err as lst.clone() does not live long enough
    	box_lst = Cons(game.store_win(), Box::new(box_lst.clone()));
	}
	println!("============\nAll games:\n{}", box_lst);
}
