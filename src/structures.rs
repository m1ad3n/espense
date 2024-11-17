
#[derive(Debug)]
pub struct User {
	id: i32,
	name: String,
}

impl User {
	pub fn new(id_a: i32, name_a: String) -> Self {
		User {
			id: id_a,
			name: name_a,
		}
	}
}

#[derive(Debug)]
pub struct Category {
	id: i32,
	name: String,
	description: String
}

impl Category {
	pub fn new(id_a: i32, name_a: String, desc_a: String) -> Self {
		Category {
			id: id_a,
			name: name_a,
			description: desc_a,
		}
	}
}