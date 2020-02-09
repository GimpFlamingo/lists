use std::mem;

pub struct List {
	head: Link,
}

enum Link {
	Empty,
	More(Box<Node>),
}

struct Node {
	elem: i32,
	next: Link,
}

impl List {
	/// # Pushes a node onto the list
	pub fn push(&mut self, elem: i32) {
		let new_node = Box::new(Node {
			elem: elem,
			next: mem::replace(&mut self.head, Link::Empty),
		});

		self.head = Link::More(new_node);
	}

	/// # Removes a node from the front of a list
	pub fn pop(&mut self) -> Option<i32> {
		match mem::replace(&mut self.head, Link::Empty) {
			Link::Empty	=> None,
			Link::More(node) => {
				self.head = node.next;
				Some(node.elem)
			}
		}
	}

	pub fn new() -> Self {
		List { head: Link::Empty }
	}
}
