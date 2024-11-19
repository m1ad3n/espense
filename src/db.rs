
use crate::{structures::User, Category, Expense};
use rusqlite::{Connection, Result, params};

pub fn table_insert_with_name(conn: &Connection, name: &str, table: &str) -> Option<i32> {
	match find_id_by_name(&conn, name, table).unwrap() {
	    Some(id) => Some(id),
	    None => {
			conn.execute(
				&format!("INSERT INTO {table} (name) VALUES (?1)"),
				params![name],
			).ok();
			find_id_by_name(&conn, name, table).unwrap()
	    }
	}
}

pub fn find_id_by_name(conn: &Connection, name: &str, table: &str) -> Result<Option<i32>> {
	let mut stmt = conn.prepare(format!("SELECT id FROM {} WHERE name = ?", table).as_str())?;
	let mut rows = stmt.query([name])?;
	if let Some(row) = rows.next()? {
		let id: i32 = row.get(0)?;
		return Ok(Some(id));
	}
	Ok(None)
}

pub fn display_users(conn: &Connection) -> Result<()> {
	let mut stmt = conn.prepare("SELECT id, name FROM users")?;
	let user_iter = stmt.query_map([], |row| {
		Ok(User::new(row.get(0)?, row.get(1)?))
	})?;

	for user in user_iter {
		println!("{:?}", user.unwrap());
	}

	Ok(())
}

pub fn display_categories(conn: &Connection) -> Result<()> {
	let mut stmt = conn.prepare("SELECT id, name, description FROM categories")?;
	let categories_iter = stmt.query_map([], |row| {
		Ok(Category::new(row.get(0)?, row.get(1)?, row.get(2)?))
	})?;

	for category in categories_iter {
		println!("{:?}", category.unwrap());
	}

	Ok(())
}

pub fn insert_expense(
	conn: &Connection,
	user_id: i32,
	amount: f32,
	category_id: i32,
	desc: String,
	date: String) -> Result<()> {
	conn.execute(&format!(
		"INSERT INTO expenses (amount, date, description, user_id, category_id) VALUES (?1, ?2, ?3, ?4, ?5)"),
		params![amount, date, desc, user_id, category_id]
	)?;
	Ok(())
}

pub fn display_expenses_with_name(conn: &Connection, name: String) -> Result<()> {
	let user_id = match find_id_by_name(&conn, name.as_str(), "users").unwrap() {
	    Some(v) => v,
	    None => 0,
	};

	let mut stmt = conn.prepare("SELECT * FROM expenses WHERE user_id = ?")?;
	let expense_iter = stmt.query_map([user_id], |row| {
		Ok(Expense::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?, row.get(7)?))
	})?;

	for expense in expense_iter {
		println!("{:#?}", expense.unwrap());
	}

	Ok(())
}

pub fn initialize_db(conn: &Connection) -> Result<()> {
	conn.execute(
		"CREATE TABLE IF NOT EXISTS users (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			name TEXT NOT NULL
		);",
		[]
	)?;

	conn.execute(
		"CREATE TABLE IF NOT EXISTS categories (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			name TEXT NOT NULL
		);",
		[]
	)?;

	conn.execute(
		"CREATE TABLE IF NOT EXISTS expenses (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			amount REAL NOT NULL,
			date TEXT NOT NULL,
			description TEXT,
			user_id INTEGER NOT NULL,
			category_id INTEGER NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
			FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL
		);",
		[]
	)?;

	Ok(())
}
