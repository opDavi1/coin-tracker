// This file is a part of coin-tracker by opDavi1 licensed under the GPL-3.0-or-later license.
// See the included LICENSE.md file for more details or go to <https://www.gnu.org/licenses/>
// 
// Part of this code is taken from RandomEngy/tauri-sqlite on github, which has no license.
// If anybody has an issue with this, please contact me.

use std::{fs, path::Path};

use rusqlite::{params, Connection};
use tauri::{AppHandle, Manager};
use crate::coin::Coin;

const DATABASE_SQL: &str = "CREATE TABLE IF NOT EXISTS coins (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        numista_id INTEGER,
        name TEXT NOT NULL,
        coin_type INT,
        issuer TEXT,
        country TEXT,
        min_year INT,
        max_year INT,
        composition TEXT,
        shape INT,
        diameter REAL,
        thickness REAL,
        weight REAL,
        orientation INT,
        denomination TEXT,
        value REAL,
        value_numerator INT,
        value_denominator INT,
        currency TEXT,
        grade INT,
        obverse_image TEXT,
        reverse_image TEXT,
        obverse_description TEXT,
        reverse_description TEXT,
        is_demonitized INT,
        comments TEXT)";

const INSERT_SQL: &str = "INSERT INTO coins  \
        VALUES (NULL, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

const UPDATE_SQL: &str = "UPDATE coins SET \
        numista_id = ?, \
        name = ?, \
        coin_type = ?, \
        issuer = ?, \
        country = ?, \
        min_year = ?, \
        max_year = ?, \
        composition = ?, \
        shape = ?, \
        diameter = ?, \
        thickness = ?, \
        weight = ?, \
        orientation = ?, \
        denomination = ?, \
        value = ?, \
        value_numerator = ?, \
        value_denominator = ?, \
        currency = ?, \
        grade = ?, \
        obverse_image = ?, \
        reverse_image = ?, \
        obverse_description = ?, \
        reverse_description = ?, \
        is_demonitized = ?, \
        comments = ? \
        WHERE id = ?";


pub fn init(app_handle: &AppHandle, file: Option<&Path>) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle.path().app_data_dir().expect("The app data dir should exist");
    fs::create_dir_all(&app_dir).expect("The app data dir should be created");
    let sqlite_path = match file {
        Some(f) => app_dir.join(f),
        None => app_dir.join("coin-tracker.sqlite"),
    };
    let db = Connection::open(sqlite_path)?;
    db.execute(DATABASE_SQL, ())?;
    println!("Initialized the database");
    Ok(db)
}


pub fn delete_coin(connection: &Connection, id: &i64) -> Result<usize, rusqlite::Error> {
    let mut statement = connection.prepare("DELETE * FROM coins WHERE id = ?")?;
    statement.execute([id])
}


pub fn get_all_coins(connection: &Connection) -> Result<Vec<Coin>, rusqlite::Error> {
    let mut statement = connection.prepare("SELECT * FROM coins")?;
    let mut rows = statement.query([])?;
    let mut coins: Vec<Coin> = Vec::new();

    while let Some(row) = rows.next()? {
         coins.push(Coin::from_sql_row(row)?);
    }

    Ok(coins)
}


pub fn get_coin_by_id(connection: &Connection, id: &i64) -> Result<Coin, rusqlite::Error> {
    let mut statement = connection.prepare("SELECT * FROM coins WHERE id = ?")?;
    statement.query_row([id], |row| {
        Coin::from_sql_row(row)
    })
}


// A count less than 1 will return all matches
pub fn get_coins_by_numista_id(connection: &Connection, numista_id: &i64, count: &i64) -> Result<Vec<Coin>, rusqlite::Error> {
    let mut statement = connection.prepare("SELECT * FROM coins WHERE numista_id = ?")?;
    let mut rows = statement.query([numista_id])?;
    let mut coins = Vec::new();
    while let Some(row) = rows.next()? {
        coins.push(Coin::from_sql_row(row)?);
    }

    if *count <  1 {
        Ok(coins)
    } else {
        Ok(coins.into_iter().take(*count as usize).collect())
    }
}


pub fn insert_coin(connection: &Connection, coin: &Coin) -> Result<i64, rusqlite::Error> {
    let mut statement = connection.prepare(INSERT_SQL)?;
    let c = coin;
    statement.insert(params!(
        c.numista_id, 
        c.name, 
        c.coin_type as i64, 
        c.issuer, 
        c.country, 
        c.min_year, 
        c.max_year, 
        c.composition, 
        c.shape as i64, 
        c.diameter, 
        c.thickness, 
        c.weight,
        c.orientation as i64,
        c.denomination,
        c.value,
        c.value_numerator,
        c.value_denominator,
        c.currency,
        c.grade,
        c.obverse_image,
        c.reverse_image,
        c.obverse_description,
        c.reverse_description,
        c.is_demonetized,
        c.comments,
    ))
}


pub fn update_coin(connection: &Connection, id: &i64, new_coin: &Coin) -> Result<(), rusqlite::Error> {
    let mut statement = connection.prepare(UPDATE_SQL)?;
    match statement.execute(params!(
        new_coin.numista_id, 
        new_coin.name, 
        new_coin.coin_type as i64, 
        new_coin.issuer, 
        new_coin.country, 
        new_coin.min_year, 
        new_coin.max_year, 
        new_coin.composition, 
        new_coin.shape as i64, 
        new_coin.diameter, 
        new_coin.thickness, 
        new_coin.weight,
        new_coin.orientation as i64,
        new_coin.denomination,
        new_coin.value,
        new_coin.value_numerator,
        new_coin.value_denominator,
        new_coin.currency,
        new_coin.grade,
        new_coin.obverse_image,
        new_coin.reverse_image,
        new_coin.obverse_description,
        new_coin.reverse_description,
        new_coin.is_demonetized,
        new_coin.comments,
        id, 
    )) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
