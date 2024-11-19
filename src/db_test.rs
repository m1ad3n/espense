
use std::any;

use crate::db::*;
use rusqlite::{Connection, Result};

fn setup_test() -> Result<Connection> {
	let conn = rusqlite::Connection::open_in_memory()?;
	initialize_db(&conn)?;
	table_insert_with_name(&conn, "Steve", "users");
	table_insert_with_name(&conn, "Molly", "users");
	table_insert_with_name(&conn, "Jack", "users");
	table_insert_with_name(&conn, "food", "categories");
	table_insert_with_name(&conn, "car payments", "categories");
	table_insert_with_name(&conn, "school", "categories");
	table_insert_with_name(&conn, "fun", "categories");
	Ok(conn)
}

#[test]
fn find_user_id_test() -> Result<()> {
	let conn = setup_test()?;
	assert_eq!(find_id_by_name(&conn, "Steve", "users")?, Some(1));
	assert_eq!(find_id_by_name(&conn, "Molly", "users")?, Some(2));
	assert_eq!(find_id_by_name(&conn, "Jack", "users")?, Some(3));
	Ok(())
}

#[test]
fn find_category_id_test() -> Result<()> {
	let conn = setup_test()?;
	assert_eq!(find_id_by_name(&conn, "food", "categories")?, Some(1));
	assert_eq!(find_id_by_name(&conn, "car payments", "categories")?, Some(2));
	assert_eq!(find_id_by_name(&conn, "school", "categories")?, Some(3));
	assert_eq!(find_id_by_name(&conn, "fun", "categories")?, Some(4));
	Ok(())
}

#[test]
fn insert_user_test() -> Result<()> {
	let conn = setup_test()?;
	assert_eq!(table_insert_with_name(&conn, "Kate", "users"), Some(4));
	Ok(())
}

#[test]
fn insert_category_test() -> Result<()> {
	let conn = setup_test()?;
	assert_eq!(table_insert_with_name(&conn, "Food", "categories"), Some(5));
	Ok(())
}