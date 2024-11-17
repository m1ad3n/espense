#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod structures;
mod db;
mod db_test;

use structures::*;
use db::*;

use std::env;
use std::{fs::{create_dir_all, File}, io::Write, path::Path, process::Stdio};
use rusqlite::{params, Connection, Result};
use std::io::{stdin, stdout};

fn get_input(prompt: &str) -> String {
	let mut buf = String::new();
	print!("{prompt}");
	stdout().flush().unwrap();
	stdin().read_line(&mut buf).unwrap();
	buf.trim().to_string()
}

pub fn get_path() -> Option<std::path::PathBuf> {
    if let Some(home_dir) = env::var("HOME").ok() {
        let path = Path::new(&home_dir).join(".espense");
        if !path.exists() {
            std::fs::create_dir_all(&path).ok()?;
        }
        Some(path)
    } else {
        None
    }
}

fn espense_list_expenses() {
	todo!();
}

fn espense_add_expense() {
	todo!()
}

fn espense_menu() {
	let buf = get_input("action >> ");
	if buf.contains("add") {
		espense_add_expense();
	}
	else if buf.contains("list") {
		espense_list_expenses();
	}
}

fn main() -> Result<()> {
	let db_folder = match get_path() {
	    Some(path) => path,
	    None => panic!("couldn't retrive your home path"),
	};
	let db_file = db_folder.join("data.db");
	dbg!(db_file);

	let conn = Connection::open_in_memory()?;
	initialize_db(&conn)?;

	espense_menu();

	Ok(())
}