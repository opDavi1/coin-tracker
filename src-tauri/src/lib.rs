// This file is a part of coin-tracker by opDavi1 licensed under the GPL-3.0-or-later license.
// See the included LICENSE.md file for more details or go to <https://www.gnu.org/licenses/>

mod coin;
mod database;
mod state;

use coin::Coin;
use state::{AppState, ServiceAccess};
use tauri::{AppHandle, Builder, Manager, State};

#[tauri::command]
fn db_delete_coin(app_handle: AppHandle, id: i64) -> Result<usize, String> {
    app_handle.db(|db| database::delete_coin(db, &id))
    .map_err(|err| err.to_string())
}

#[tauri::command]
fn db_get_all_coins(app_handle: AppHandle) -> Result<Vec<Coin>, String> {
    app_handle.db(|db| database::get_all_coins(db))
    .map_err(|err| err.to_string())
}

#[tauri::command]
fn db_get_coin_by_id(app_handle: AppHandle, id: i64) -> Result<Coin, String> {
    app_handle.db(|db| database::get_coin_by_id(db, &id))
    .map_err(|err| err.to_string())
}

#[tauri::command]
fn db_get_coins_by_numista_id(app_handle: AppHandle, numista_id: i64, count: i64) -> Result<Vec<Coin>, String> {
    app_handle.db(|db| database::get_coins_by_numista_id(db, &numista_id, &count))
    .map_err(|err| err.to_string())
}

#[tauri::command]
fn db_insert_coin(app_handle: AppHandle, coin: Coin) -> Result<i64, String> {
    app_handle.db(|db| database::insert_coin(db, &coin))
    .map_err(|err| err.to_string())

}

#[tauri::command]
fn db_update_coin(app_handle: AppHandle, id: i64, new_coin: Coin) -> Result<(), String> {
    app_handle.db(|db| database::update_coin(db, &id, &new_coin))
    .map_err(|err| err.to_string())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .manage(AppState {db: Default::default() })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            db_delete_coin,
            db_get_all_coins,
            db_get_coin_by_id,
            db_get_coins_by_numista_id,
            db_insert_coin,
            db_update_coin,
            ])
        .setup(|app| {
            let handle = app.handle();
            let app_state: State<AppState> = handle.state();
            let db = database::init(&handle, None).expect("Could not initialize database");
            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    
}
