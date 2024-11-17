#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod user;
mod db;
mod db_test;

use user::*;
use db::*;

use std::{fs::{create_dir_all, File}, path::Path};
use rusqlite::{params, Connection, Result};

fn initialize_path(folder: &str, filename: &str) {
	if !Path::new(folder).exists() {
		create_dir_all(folder).unwrap();
	}

	if !Path::new(filename).exists() {
		File::create(filename).unwrap();
	}
}

fn main() -> Result<()> {
	let db_folder = "/home/mladen/.cache/espense";
	let db_file = format!("{}/data.db", db_folder);
	initialize_path(db_folder, &db_file);

	let conn = Connection::open_in_memory()?;
	initialize_db(&conn)?;

	insert_user(&conn, "Jack")?;
	insert_user(&conn, "steve")?;
	insert_category(&conn, "food", "spent on food")?;

	display_users(&conn)?;
	display_categories(&conn)?;

	Ok(())
}