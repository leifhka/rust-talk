extern crate rand;

use std::io;
use std::fmt;
use std::option::*;
use std::rc::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Mark {
	O,
	X,
	E,
}

#[derive(Debug)]
struct TicTacToe {
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

impl fmt::Display for TicTacToe {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = String::from("");
		let mut b = String::from("");

		for i in 0..3 {
			let ind = i*3;
			s = s + format!("{}\n│{}│{}│{}│\n", b,
			                self.board[ind],
			                self.board[ind+1],
			                self.board[ind+2]).as_str();
			b = String::from("├─┼─┼─┤");
		}

		write!(f, "┌─┬─┬─┐{}└─┴─┴─┘", s)
	}
}

impl TicTacToe {
	fn new() -> Self {
		use Mark::*;
		TicTacToe { board: [E; 9], turn: X, winner: E }
	}

	fn make_player_move(&mut self) -> bool {
		match self.turn {
			Mark::X => self.make_human_move(),
			_ => self.make_ai_move(),
		}
	}

	fn place_mark(&mut self, m: Mark, i: usize) -> bool {
		if self.board[i] == Mark::E && i < 9 {
			self.board[i] = m;
			self.flip_turn();
			true
		} else {
			false
		}
	}

	fn flip_turn(&mut self) {
		match self.turn {
			Mark::X => self.turn = Mark::O,
			_ => self.turn = Mark::X,
		}
	}

	fn make_human_move(&mut self) -> bool {
		println!("{}", &self);
		println!("Make a move:");
		let mut input_text = String::new();
		io::stdin()
		    .read_line(&mut input_text)
		    .expect("failed to read from stdin");
		let trimmed = input_text.trim();
		if let Ok(i) = trimmed.parse::<usize>() {
			self.place_mark(Mark::X, i)
		} else {
			false
		}
	}

	fn make_ai_move(&mut self) -> bool {
		use rand::distributions::{IndependentSample, Range};

		let mut rng = rand::thread_rng();
		let between = Range::new(0, 9);
		self.place_mark(Mark::O, between.ind_sample(&mut rng))
	}

	fn flip(&self) -> Self {
		let mut flipped = [Mark::E; 9];
		for i in 0..3 {
			for j in 0..3 {
				flipped[3*i + (2-j)] = self.board[3*j+i];
			}
		}
		return TicTacToe {board: flipped, winner: self.winner, turn: self.turn};
	}

	fn is_won_horizontal(&self) -> Option<Mark> {
		let b = &self.board;
		is_won_row(&b[0..3])
    	.or(is_won_row(&b[3..6]))
    	.or(is_won_row(&b[6..9]))
    	.or(is_won_row(&[b[0], b[4], b[8]]))
	}

	fn is_won(&mut self) -> bool {
		if let Some(winner) = self.is_won_horizontal()
		                      .or(self.flip().is_won_horizontal()) {
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

	fn consume(self) -> String {
		format!("{}", self)
	}
}

fn is_won_row(row: &[Mark]) -> Option<Mark> {
	if row[0] != Mark::E &&
	   row[0] == row[1] &&
	   row[1] == row[2] {
		Some(row[0])
	} else {
		None
	}
}

fn play_game() -> TicTacToe {
	let mut game = TicTacToe::new();
	while !game.is_done() {
		while !game.make_player_move() {}
	}
	println!("============\n{}", &game);
	match game.winner {
		Mark::E => println!("Tie!"),
		_ => println!("Player {} won!", game.winner),
	}
	//let g = game; // Leads to move error
	//game.consume(); // Leads to move error
	return game;
}

#[derive(Debug)]
struct Score {
	xwin: u32,
	owin: u32,
}

impl Score {
	fn add_game(&mut self, game: TicTacToe) {
		match game.winner {
			Mark::O => self.owin += 1,
			Mark::X => self.xwin += 1,
			_ => ()
		};
	}
}

// Infinite size, does not compile
//enum ErrList<E> {
//	Nil,
//	Cons(E, ErrList),
//}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum RefList<'a, E: 'a + fmt::Display + Clone> {
	Nil,
	Cons(E, &'a RefList<'a, E>),
}

impl<'a, E: fmt::Display + Clone> RefList<'a, E> {
	fn add(&'a self, e: E) -> RefList<'a, E> {
		RefList::Cons(e, &self)
	}
}

#[derive(Debug, Clone)]
enum BoxList<E> {
	Nil,
	Cons(E, Box<BoxList<E>>),
}

struct Node<E> {
	e: E,
	nodes: Vec<Rc<Node<E>>>,
}


fn main() {
	use BoxList::*;
	let mut score = Score { xwin: 0, owin: 0 };
	for _ in 0..3 {
		score.add_game(play_game());
	}
	println!("============\n{:?}", score);

	// let mut n1 = Node { e: 1, nodes: Vec::new() };
	// let mut n2 = Node { e: 2, nodes: Vec::new() };
	// let n1r = Rc::new(n1);
	// n2.nodes.push(n1r.clone());
	// let mut n3 = Node { e: 3, nodes: Vec::new() };
	// n3.nodes.push(n1r.clone());
	// Rc::get_mut(&mut n2.nodes[0]).unwrap().e = 5;
	// println!("{}, {}", n2.nodes[0].e, n1r.e);

	//let mut x = 5;
	//let raw = &mut x as *mut i32;
	// println!("{}", *raw); // Unsafe, wont work
	//unsafe {
	//	println!("{}", *raw); // Unsafe, works in unsafe
	//	let raw2 = raw;
	//	*raw2 = 2;
	//	*raw = 1;
	//	println!("{},{}", *raw2, *raw); // Unsafe, works in unsafe
	//}
}

