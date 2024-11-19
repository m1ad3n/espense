
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

#[derive(Debug)]
pub struct Expense {
	id: i32,
	amount: f32,
	date: String,
	desc: String,
	user_id: i32,
	category_id: i32,
	created_at: String,
	updated_at: String,
}

impl Expense {
	pub fn new(id_a: i32, amount_a: f32, date_a: String, desc_a: String, user_id_a: i32, category_id_a: i32, created_at_a: String, updated_at_a: String) -> Expense {
		Expense {
			id: id_a,
			amount: amount_a,
			date: date_a,
			desc: desc_a,
			user_id: user_id_a,
			category_id: category_id_a,
			created_at: created_at_a,
			updated_at: updated_at_a,
		}
	}
}