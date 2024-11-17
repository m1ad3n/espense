
use crate::{user::User, Category};
use rusqlite::{Connection, Result, params};

pub fn insert_user(conn: &Connection, name: &str) -> Result<()> {
	conn.execute(
		"INSERT INTO users (name) VALUES (?1)",
		params![name],
	)?;
	
	Ok(())
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

pub fn insert_category(conn: &Connection, name: &str, desc: &str) -> Result<()> {
	conn.execute(
		"INSERT INTO categories (name, description) VALUES (?1, ?2)",
		params![name, desc],
	)?;

	Ok(())
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
			name TEXT NOT NULL,
			description TEXT NOT NULL
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
