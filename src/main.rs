#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod structures;
mod db;
mod cli;

use structures::*;
use db::*;
use cli::*;

use std::env;
use std::fs::{create_dir_all, File};
use std::path::Path;
use rusqlite::{params, Connection, Result};
use std::io::{stdin, stdout};

pub fn get_path() -> Option<std::path::PathBuf> {
    if let Some(home_dir) = env::var("HOME").ok() {
        let path = Path::new(&home_dir).join(".espense");
        if !path.exists() {
            std::fs::create_dir_all(&path).ok()?;
        }
        return Some(path)
    }
	None
}

fn espense_list_expenses(conn: &Connection) {
	let inp_name = get_input("name >> ");
	display_expenses_with_name(&conn, inp_name).unwrap();
}

fn espense_add_expense(conn: &Connection) {
	let user_id = table_insert_with_name(&conn, get_input("name >> ").as_str(), "users").unwrap();
	let amount = get_input("amount >> ")
					.parse::<f32>()
					.expect("please enter the decimal value");
	let category_id = table_insert_with_name(&conn, get_input("category >> ").as_str(), "categories").unwrap();
	let desc = get_input("description >> ");
	let date = get_input("date >> ");
	insert_expense(&conn, user_id, amount, category_id, desc, date).unwrap();
}

fn espense_menu(conn: &Connection) {
	let buf = get_input("action >> ");
	if buf.contains("add") {
		espense_add_expense(&conn);
	}
	else if buf.contains("list") {
		espense_list_expenses(&conn);
	}
}

fn main() -> Result<()> {
	let db_folder = match get_path() {
	    Some(path) => path,
	    None => panic!("couldn't retrive your home path"),
	};
	let db_file = db_folder.join("data.db").into_os_string();

	let conn = Connection::open(db_file)?;
	initialize_db(&conn)?;

	espense_menu(&conn);

	Ok(())
}