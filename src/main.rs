use std::thread;
use std::sync::{Arc, Mutex};

struct Philosopher {
	name: String,
	left: usize,
	right: usize,
}

struct Table {
	forks: Vec<Mutex<()>>,
}

impl Philosopher {
	fn new(name: &str, left: usize, right: usize) -> Philosopher {
		Philosopher {
			name: name.to_string(),
			left: left,
			right: right,
		}
	}

	fn eat(&self, table: &Table) -> &Philosopher {
		let _left  = table.forks[self.left].lock().unwrap();

		thread::sleep_ms(1000);

		let _right = table.forks[self.right].lock().unwrap();

		println!("{} started eating.", self.name);

		println!("{} is done eating.", self.name);

		self
	}
}

fn main() {
	let table = Arc::new(Table { forks: vec![
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
	]});

	let philosophers = vec![
		Philosopher::new("Socrates", 0, 1),
		Philosopher::new("Aristotle", 1, 2),
		Philosopher::new("Xenophon", 2, 3),
		Philosopher::new("Plato", 3, 4),
		Philosopher::new("Pythagoras", 0, 4),
	];

	let handles: Vec<_> = philosophers.into_iter().map(|p| {
		let table = table.clone();

		thread::spawn(move || {
			p.eat(&table);
		})
	}).collect();

	for handle in handles {
		handle.join().unwrap();
	}
}
