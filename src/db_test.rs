
use crate::db::*;
use rusqlite::{Connection, Result};

fn setup_test() -> Result<Connection> {
	let conn = rusqlite::Connection::open_in_memory()?;
	initialize_db(&conn)?;
	insert_user(&conn, "Steve")?;
	insert_user(&conn, "Molly")?;
	insert_user(&conn, "Jack")?;
	insert_category(&conn, "food", "")?;
	insert_category(&conn, "car payments", "")?;
	insert_category(&conn, "school", "")?;
	insert_category(&conn, "fun", "")?;
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
	assert_eq!(insert_user(&conn, "Kate"), Ok(()));
	Ok(())
}

#[test]
fn insert_category_test() -> Result<()> {
	let conn = setup_test()?;
	assert_eq!(insert_category(&conn, "Food", "food stuff"), Ok(()));
	Ok(())
}